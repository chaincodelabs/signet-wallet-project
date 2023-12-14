from random import choice, randrange
from requests import get
from pathlib import Path
from threading import Thread
from time import sleep
import json
import os
import subprocess
import sys

# Faster than signet
REGTEST=False

if len(sys.argv) < 2 or len(sys.argv) > 4:
    raise Exception("Args: <path to bitcoin core repo> <optional: path to output directory> <optional: path to signet datadir>")
# Destination for bitcoin datadir
DATA_DIR = None
if len(sys.argv) == 4:
    DATA_DIR = sys.argv.pop()
# Destination for files for students (bitcoin.conf and wallet descriptors)
CONF_DIR = Path(os.path.dirname(os.path.abspath(__file__))) / "config"
if len(sys.argv) == 3:
    CONF_DIR = Path(sys.argv.pop())
    os.makedirs(f"{CONF_DIR}", exist_ok=True)
# Import Bitcoin Core test framework as a library
repo = Path(sys.argv.pop())
if not repo.exists():
    raise Exception(f"{repo} does not exist")
sys.path.insert(0, f"{repo / 'test' / 'functional'}")
from test_framework.test_shell import TestShell
from test_framework.util import p2p_port
from test_framework.authproxy import AuthServiceProxy

# Ensure that all RPC calls are made with brand new http connections
def auth_proxy_request(self, method, path, postdata):
    self._set_conn() # creates new http client connection
    return self.oldrequest(method, path, postdata)
AuthServiceProxy.oldrequest = AuthServiceProxy._request
AuthServiceProxy._request = auth_proxy_request

EXTERNAL_IP = get('https://api.ipify.org').content.decode('utf8')
NUM_WALLETS = 500

# Parse listdescriptors
def get_wpkh(wallet_rpc):
    info = wallet_rpc.listdescriptors(private=True)
    for item in info["descriptors"]:
        if not item["internal"] and item["desc"].startswith("wpkh"):
            return item
    return None

# Setup regtest test shell, create wallet and signet challenge
shell = TestShell().setup(num_nodes=1, setup_clean_chain=True)
node = shell.nodes[0]
node.createwallet(wallet_name="signer")
descs = node.listdescriptors(private=True)["descriptors"]
for desc in descs:
    desc["timestamp"] = 0
print(descs)
addr = node.getnewaddress(address_type="bech32")
signet_challenge = node.getaddressinfo(addr)["scriptPubKey"]
shell.log.info(f"Descriptors: {json.dumps(desc)}\n")
shell.log.info(f"Signet Challenge: {signet_challenge}\n")

# Done with regtest
shell.shutdown()

# Start signet
shell = TestShell().setup(
    num_nodes=1,
    setup_clean_chain=True,
    chain="regtest" if REGTEST else "signet",
    extra_args=[[f"-signetchallenge={signet_challenge}", f"-bind=0.0.0.0", f"-txindex"]],
    tmpdir=DATA_DIR,
    rpc_timeout=600)
# Keep eveything for restarts
shell.options.nocleanup = True

# Create mining node
node = shell.nodes[0]
node.createwallet(wallet_name="miner")
miner_wallet = node.get_wallet_rpc("miner")
miner_wallet.importdescriptors(descs)
miner_addr = miner_wallet.getnewaddress(address_type="bech32")

# Add challenge to conf file for future restarts
conf_file = node.datadir_path / "bitcoin.conf"
with open(conf_file, "a") as f:
    f.write(f"signetchallenge={signet_challenge}\n")

# Create wallets, cache p2wpkh descriptors and RPC wrappers
wallets = []
for i in range(NUM_WALLETS):
    name = f"wallet_{i:03}"
    node.createwallet(wallet_name=name)
    rpc = node.get_wallet_rpc(name)
    desc = get_wpkh(rpc)
    wallets.append({"name": name, "rpc": rpc, "desc": desc})
    shell.log.info(f"Created wallet: {name} {desc}")

# Export config data
with open(f"{CONF_DIR / 'bitcoin.conf'}", "w") as f:
    f.write("signet=1\n")
    f.write("[signet]\n")
    f.write(f"signetchallenge={signet_challenge}\n")
    f.write(f"addnode={EXTERNAL_IP}:{p2p_port(node.index)}\n")

with open(f"{CONF_DIR / 'wallets.txt'}", "w") as f:
    for wallet in wallets:
        f.write(f"{wallet['name']}: {wallet['desc']['desc']}\n")


# Start mining in background subprocess
if REGTEST:
    shell.log.info(f"Generating blocks: {node.generatetoaddress(101, address=miner_addr, invalid_call=False)}")
    def generate_regtest():
        while True:
            try:
                shell.log.info(f"Generated block: {node.generatetoaddress(1, address=miner_addr, invalid_call=False)}")
                sleep(1)
            except Exception as e:
                shell.log.info(f"Couldn't generate block: {e}")
    Thread(target=generate_regtest).start()
else:
    miner = f"{repo / 'contrib' / 'signet' / 'miner'}"
    grinder = f"{repo / 'src' / 'bitcoin-util'}"
    cmd = [
        sys.executable,
        miner,
        f"--cli={node.cli.binary} -datadir={node.cli.datadir} -rpcwallet=miner",
        "generate",
        f"--address={miner_addr}",
        f"--grind-cmd={grinder} grind",
        "--min-nbits",
        "--ongoing"]
    subprocess.Popen(cmd, stderr=subprocess.STDOUT)

# Start a thread where the miner funds wallets whenever it can
def maybe_fund_wallets():
    while True:
        if node.getblockcount() >= 300:
            return
        # Wait for mature coins
        try:
            bal = miner_wallet.getbalances()
            trusted = bal["mine"]["trusted"]
            immature = bal["mine"]["immature"]
            shell.log.info(f"Miner wallet balance: trusted={trusted} immature={immature}")
            if trusted < 1:
                sleep(10)
                continue
        except Exception as e:
            shell.log.info(f"Failed to get miner wallet balance: {e}")
            sleep(10)
            continue
        # Fund wallets
        try:
            outputs = {}
            for wallet in wallets:
                addr = wallet["rpc"].getnewaddress()
                amt = round((float(trusted) * 0.9) / len(wallets), 8)
                outputs[addr] = amt
            tx = miner_wallet.sendmany("", outputs)
            shell.log.info(f"Sending tx from miner to all wallets: {tx}")
        except Exception as e:
            shell.log.info(f"Failed to send tx from miner to all wallets: {e}")
        sleep(10)
Thread(target=maybe_fund_wallets).start()

# Start a thread where random wallets send random TXs to each other
def tx_blizzard():
    while True:
        try:
            if node.getblockcount() >= 290:
                return
            for wallet in wallets:
                bal = wallet["rpc"].getbalance()
                if bal < 1:
                    continue
                dest = choice(wallets)
                chng = wallet["rpc"].getnewaddress()
                addr = dest["rpc"].getnewaddress()
                amt = randrange(10000, int(bal * 100000000)) / 100000000
                tx = wallet["rpc"].send(outputs={addr: amt}, options={"change_address": chng})
                shell.log.info(f"Sending tx: from {wallet['name']} to {dest['name']}: {tx['txid']}")
        except Exception as e:
            shell.log.info(f"Failed to send tx: from {wallet['name']} to {dest['name']}: {e}")
        sleep(10)
Thread(target=tx_blizzard).start()
