# MPC Solana Example  

This project demonstrates a **Multi-Party Computation (MPC)** signing workflow for Solana transactions using EdDSA.  

## Local Testing
The following commands were used for local testing.

```
# ----------------------
# Step 1: Generate secret shares (Party A & Party B)
# ----------------------

cargo run generate
# Output (Party A):
# secret share: 2qwAaF5UiFGkaeajrKi7RYDhxiAu1XKsQzZXXvTwCVrs5W87Quft6c95AczLEAMCstV1W9pUji5ahgC3g4mW1bMp
# public share: AMrngtzHbeMgAUe55GxJFfPVCsGB2mbfruXAZKrnhahk

cargo run generate
# Output (Party B):
# secret share: 3DQg4bM4hQ5MqwCqqr9L6VvBGQkG71vov5e6ySEPAK4yHs1u5ZDkqToP31VaDwAtR3fWPwbZ96wA7WwQbDCaaXeD
# public share: FauJkfn3DnWcQnJ4UKjg6oCrL5VdfHFxfyRtU2UKByem


# ----------------------
# Step 2: Aggregate public keys
# ----------------------

cargo run aggregate-keys \
  AMrngtzHbeMgAUe55GxJFfPVCsGB2mbfruXAZKrnhahk \
  FauJkfn3DnWcQnJ4UKjg6oCrL5VdfHFxfyRtU2UKByem
# Output:
# The Aggregated Public Key: 9khbh6uU4FDahRB1Zh2rUWrXvoxBFhWsLJuie4Sw84Hc


# ----------------------
# Step 3: Generate commitments (send step one)
# ----------------------

cargo run agg-send-step-one 2qwAaF5UiFGkaeajrKi7RYDhxiAu1XKsQzZXXvTwCVrs5W87Quft6c95AczLEAMCstV1W9pUji5ahgC3g4mW1bMp
# Output (Party A):
# Message 1: 1iktSP74RdCXNXEZ1ouKVLu...
# Secret state: 2hm996bK1qYHoWPvmkuStPWt...

cargo run agg-send-step-one 3DQg4bM4hQ5MqwCqqr9L6VvBGQkG71vov5e6ySEPAK4yHs1u5ZDkqToP31VaDwAtR3fWPwbZ96wA7WwQbDCaaXeD
# Output (Party B):
# Message 1: 1jByuFu69azbp5nxpZQLFcw...
# Secret state: 2cPGwXX7WP5jghL1JXFsyWp...


# ----------------------
# Step 4: Generate partial signatures (send step two)
# ----------------------

# Party A
cargo run agg-send-step-two \
  --keypair 2qwAaF5UiFGkaeajrKi7RYDhxiAu1XKsQzZXXvTwCVrs5W87Quft6c95AczLEAMCstV1W9pUji5ahgC3g4mW1bMp \
  --amount 15 \
  --to 5YLbUx2MGaHvSV1de5Kr1dVWPupbf63Mm5a9VhtvqoNt \
  --keys AMrngtzHbeMgAUe55GxJFfPVCsGB2mbfruXAZKrnhahk FauJkfn3DnWcQnJ4UKjg6oCrL5VdfHFxfyRtU2UKByem \
  --first-messages 1jByuFu69azbp5nxpZQLFcw... \
  --secret-state 2hm996bK1qYHoWPvmkuStPWt... \
  --recent-block-hash DK48CqXTSBBgTHhh5Wzp16YxDqUqSGXxhNN6TktR5h3j
# Output:
# Partial signature: D5MdioktovJ13gSRdnZr8AZyxshZ2MDxLkZKRhkZc2NCSM1n327GVsyv6TiUm3h7isZAq9qsvJ7xaiHsGpaj9r73


# Party B
cargo run agg-send-step-two \
  --keypair 3DQg4bM4hQ5MqwCqqr9L6VvBGQkG71vov5e6ySEPAK4yHs1u5ZDkqToP31VaDwAtR3fWPwbZ96wA7WwQbDCaaXeD \
  --amount 15 \
  --to 5YLbUx2MGaHvSV1de5Kr1dVWPupbf63Mm5a9VhtvqoNt \
  --keys AMrngtzHbeMgAUe55GxJFfPVCsGB2mbfruXAZKrnhahk FauJkfn3DnWcQnJ4UKjg6oCrL5VdfHFxfyRtU2UKByem \
  --first-messages 1iktSP74RdCXNXEZ1ouKVLu... \
  --secret-state 2cPGwXX7WP5jghL1JXFsyWp... \
  --recent-block-hash DK48CqXTSBBgTHhh5Wzp16YxDqUqSGXxhNN6TktR5h3j
# Output:
# Partial signature: D5MdioktovJ13gSRdnZr8AZyxshZ2MDxLkZKRhkZc2NCFvLepPmFJYY1nk1CiRQWVR4MLYz2FFXohdtzb3kkr6MN


# ----------------------
# Step 5: Aggregate signatures and broadcast transaction
# ----------------------

cargo run aggregate-signatures-and-broadcast \
  --signatures D5MdioktovJ13gSRdnZr8AZyxshZ2MDxLkZKRhkZc2NCSM1n327GVsyv6TiUm3h7isZAq9qsvJ7xaiHsGpaj9r73 \
               D5MdioktovJ13gSRdnZr8AZyxshZ2MDxLkZKRhkZc2NCFvLepPmFJYY1nk1CiRQWVR4MLYz2FFXohdtzb3kkr6MN \
  --amount 0.1 \
  --to 5YLbUx2MGaHvSV1de5Kr1dVWPupbf63Mm5a9VhtvqoNt \
  --recent-block-hash DK48CqXTSBBgTHhh5Wzp16YxDqUqSGXxhNN6TktR5h3j \
  --net devnet \
  --keys A7nKLM8KbXrgrZ4CPjA9pkS1QsdyX4aPbeBKvMRdkZW6 6TLKjCNFKVHf5X8ypNK9BmLWiwNDu3q4ZhDauJvumt3c
```
