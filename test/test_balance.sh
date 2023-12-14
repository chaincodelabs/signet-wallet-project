result=$(OPENSSL_CONF=./test/openssl.cnf bash ./solution/run_balance.sh | tail -n 1)
echo $result
grep -xq "$result" ./test/wallet_balances.txt && echo "PASS" || echo "FAIL"
