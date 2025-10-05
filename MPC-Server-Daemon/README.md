# Solana Threshold Signatures PoC
A Proof-Of-Concept showing n-of-n offchain multisignatures on Solana

## Curl commands to interact with the backend


```

----------------------
Step 1: Generate secret shares (Party A & Party B)
----------------------

curl -X POST http://127.0.0.1:3000/generate

output: {"secret_share":"3396aD13YonaRVzXEnk65XgpKNw93WDduArMb2gJgGWQLe2Q3TGGLa9SiNksagdUWLotLJX6frnAbu5u2axPt4re","public_share":"B5V94zc2kBrnKL68BjvgchGDVGbLrP8JhCHbJFT9j6z"}



curl -X POST http://127.0.0.1:3000/generate

output: {"secret_share":"51orn5kBTK3vcgRSGT2uFpMLkapPWR8f3jXJ4XCyoVFipnrxMad3PRe7x31cc18dVqtuRWcSC4LgYoW6owFPN5Ku","public_share":"EEUswabhdFEFeJdtCvB8sQVBaRPKNf174wa3hgvyErVV"}


----------------------
Step 2: Aggregate public keys
----------------------

curl -X POST http://127.0.0.1:3000/aggregate_keys   -H "Content-Type: application/json"   -d '{"keys":["B5V94zc2kBrnKL68BjvgchGDVGbLrP8JhCHbJFT9j6z","EEUswabhdFEFeJdtCvB8sQVBaRPKNf174wa3hgvyErVV"]}'

output: {"aggregated_key":"3HFuRypw5i2amjPZ2QhdajigD1fPMrt32GEPJMDNN24Y"}


----------------------
Step 3: Generate commitments (send step one)
----------------------

curl -X POST http://127.0.0.1:3000/agg_send_step_one -H "Content-Type:application/json" -d'{"keys": [3396aD13YonaRVzXEnk65XgpKNw93WDduArMb2gJgGWQLe2Q3TGGLa9SiNksagdUWLotLJX6frnAbu5u2axPt4re]}'

output: {"first_msg":"1iwZQkqtrp2gMTiyPHPPG69G3Yuunvyk9dPvkevDDwfCjZh1CHivB6afhn8FadNJLzFnWz4cTnzcmjLkqE4snpuzigxkBNi1pJ5ZvTu2xyppAU2oqtYpvNmab7Yecb5xc73C","secret_state":"2nmKXxkseSrtYxQF4cXC2ZTJ9eUhbi4dZtZaE2CYyzCHTKxLUbCZ3gev8wJMY72jvgvMKryvbD4urdVk4LBKaaQMaSExowaCYixXEbKXuZDpJ65PkWVN7Q4r1quFhbz8mCJjtNMfD1pRK81i7dYoaPbr3o868aAXGH5FnTzQDkWZ5n4d"}


curl -X POST http://127.0.0.1:3000/agg_send_step_one -H "Content-Type:application/json" -d'{"keys": [51orn5kBTK3vcgRSGT2uFpMLkapPWR8f3jXJ4XCyoVFipnrxMad3PRe7x31cc18dVqtuRWcSC4LgYoW6owFPN5Ku]}'

output:
{"first_msg":"1meDg1TLDJC2vssbxNTLAfadv3JuwuiHvF7HvyoQkRxjWs9GPoUzUBCVqtnie7Xcj7kEmQU1MSNBnAfhikYKkmSWDKmMeViKV2SvxkgMrEB8n3g9T43gN2dDE7Kc1E8pcM4k","secret_state":"2gBcFgobfcPpfdESRrNZana6cidkUfN3TXi4U9w4A7NtuXp3oJw7tMkqDgWckb5qo3x72M6GvwWpuJnoM4NnNyKD7scD7k9mRS8Z1tWk7GNu5Kn3iV3w9FgVytNwHKB43dtnuKU6ZS8EdPYSPRaMZEQ3V2KT9yBC8x1FJE829KLEpZxh"}


----------------------
Step 4: Generate partial signatures (send step two)
----------------------

// Message should be of a different person. For A, it should be from (B, C, D, E)
curl -X POST http://127.0.0.1:3000/agg_send_step_two -H "Content-Type:application/json" -d'{
"keypair": "3396aD13YonaRVzXEnk65XgpKNw93WDduArMb2gJgGWQLe2Q3TGGLa9SiNksagdUWLotLJX6frnAbu5u2axPt4re",
"amount": 0.5,
"to": "5YLbUx2MGaHvSV1de5Kr1dVWPupbf63Mm5a9VhtvqoNt",
"memo": "Payment for Coffee",
"recent_block_hash": "28UN5myEy7BHVcEmkFfF47cwQktukFB2aq9XrXGXgtSb",
"keys": ["B5V94zc2kBrnKL68BjvgchGDVGbLrP8JhCHbJFT9j6z", "EEUswabhdFEFeJdtCvB8sQVBaRPKNf174wa3hgvyErVV"],
"first_messages": ["1meDg1TLDJC2vssbxNTLAfadv3JuwuiHvF7HvyoQkRxjWs9GPoUzUBCVqtnie7Xcj7kEmQU1MSNBnAfhikYKkmSWDKmMeViKV2SvxkgMrEB8n3g9T43gN2dDE7Kc1E8pcM4k"],
"secret_state": "2nmKXxkseSrtYxQF4cXC2ZTJ9eUhbi4dZtZaE2CYyzCHTKxLUbCZ3gev8wJMY72jvgvMKryvbD4urdVk4LBKaaQMaSExowaCYixXEbKXuZDpJ65PkWVN7Q4r1quFhbz8mCJjtNMfD1pRK81i7dYoaPbr3o868aAXGH5FnTzQDkWZ5n4d"
}'

Output: 
{"partial_signature":"FwTLno5BJ7cUGWjRJxpV9z73K2UxxEkET7XnP4Xhb1x2PLU4RwJQCkZoDMrvi2YhGjDYKLzR1CqD89u6fNsdqwF6"}




curl -X POST http://127.0.0.1:3000/agg_send_step_two -H "Content-Type:application/json" -d'{
"keypair": "51orn5kBTK3vcgRSGT2uFpMLkapPWR8f3jXJ4XCyoVFipnrxMad3PRe7x31cc18dVqtuRWcSC4LgYoW6owFPN5Ku",
"amount": 0.5,
"to": "5YLbUx2MGaHvSV1de5Kr1dVWPupbf63Mm5a9VhtvqoNt",
"memo": "Payment for Coffee",
"recent_block_hash": "28UN5myEy7BHVcEmkFfF47cwQktukFB2aq9XrXGXgtSb",
"keys": ["B5V94zc2kBrnKL68BjvgchGDVGbLrP8JhCHbJFT9j6z", "EEUswabhdFEFeJdtCvB8sQVBaRPKNf174wa3hgvyErVV"],
"first_messages": ["1iwZQkqtrp2gMTiyPHPPG69G3Yuunvyk9dPvkevDDwfCjZh1CHivB6afhn8FadNJLzFnWz4cTnzcmjLkqE4snpuzigxkBNi1pJ5ZvTu2xyppAU2oqtYpvNmab7Yecb5xc73C"],
"secret_state": "2gBcFgobfcPpfdESRrNZana6cidkUfN3TXi4U9w4A7NtuXp3oJw7tMkqDgWckb5qo3x72M6GvwWpuJnoM4NnNyKD7scD7k9mRS8Z1tWk7GNu5Kn3iV3w9FgVytNwHKB43dtnuKU6ZS8EdPYSPRaMZEQ3V2KT9yBC8x1FJE829KLEpZxh"
}'


Output: 
{"partial_signature":"FwTLno5BJ7cUGWjRJxpV9z73K2UxxEkET7XnP4Xhb1x2Tfd4tB8hUE7rfwityCd2ygjCyJPSxX2FMT89wBp664Rp"}




----------------------
Step 5: Aggregate signatures and broadcast transaction
----------------------

curl -X POST http://127.0.0.1:3000/agg_sig_and_broadcast -H "Content-Type:application/json" -d'{
"signatures": ["FwTLno5BJ7cUGWjRJxpV9z73K2UxxEkET7XnP4Xhb1x2PLU4RwJQCkZoDMrvi2YhGjDYKLzR1CqD89u6fNsdqwF6", "FwTLno5BJ7cUGWjRJxpV9z73K2UxxEkET7XnP4Xhb1x2Tfd4tB8hUE7rfwityCd2ygjCyJPSxX2FMT89wBp664Rp"],
"amount": 0.5,
"to": "5YLbUx2MGaHvSV1de5Kr1dVWPupbf63Mm5a9VhtvqoNt",
"memo": "Payment for Coffee",
"recent_block_hash": "28UN5myEy7BHVcEmkFfF47cwQktukFB2aq9XrXGXgtSb",
"net": "https://api.devnet.solana.com",
"keys": ["B5V94zc2kBrnKL68BjvgchGDVGbLrP8JhCHbJFT9j6z", "EEUswabhdFEFeJdtCvB8sQVBaRPKNf174wa3hgvyErVV"]
}'

```
