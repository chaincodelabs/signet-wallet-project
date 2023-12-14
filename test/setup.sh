wget https://bitcoincore.org/bin/bitcoin-core-26.0/bitcoin-26.0-x86_64-linux-gnu.tar.gz
tar -xzvf bitcoin-26.0-x86_64-linux-gnu.tar.gz
ln -s $PWD/bitcoin-26.0/bin/* /usr/local/bin/
bitcoind -daemon -signet -signetchallenge=0014e465e43a5e6a013f5c5ec43eae826f57cbb3f83d -blocksonly=1 -addnode=35.209.148.157:12738
while true; do
    blockcount=$(bitcoin-cli -signet getblockcount)
    if [[ $blockcount -ge 300 ]]; then
        echo "blocks: $blockcount"
        break
    else
        sleep 1
    fi
done
bitcoin-cli -signet invalidateblock 0000009b0e57d6f42727fe4047eaa9d6be25c82dbf19d55fda56c85ad6189d58
