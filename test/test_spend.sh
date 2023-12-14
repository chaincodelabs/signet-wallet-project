result=$(OPENSSL_CONF=./test/openssl.cnf bash ./solution/run_spend.sh)
echo $result
tx1=$(echo "$result" | tail -n 2 | head -n 1)
tx2=$(echo "$result" | tail -n 1)

bitcoin-cli -signet sendrawtransaction $tx1
bitcoin-cli -signet sendrawtransaction $tx2