stellar keys generate owner
stellar keys fund alice
stellar keys fund owner

stellar contract deploy \
    --source-account=alice \
    --wasm=target/wasm32-unknown-unknown/release/token.wasm \
    -- \
    --admin SDXU6YQNO6QNZFG57PEBAXJGVD6H7XZYHXZNHA37JLZXISDT65R2ORFU \
    --decimal 18 \
    --name usdc \
    --symbol USDC

USDC=CASNAXVSC5MYE2EYPFRMF4DM42G2TG3BMCDGVDL2SKYYVJDE7UN7MKM6


stellar contract upload \
    --source=alice \
    --wasm=target/wasm32-unknown-unknown/release/company.wasm

WASM_HASH=8d45fbf03c2788e103dab335838bb71b7ab78bcc91c9d6e408bf06c88130441a


stellar contract deploy \
    --source-account=alice \
    --wasm=target/wasm32-unknown-unknown/release/paygo.wasm \
    -- \
    --usdc CASNAXVSC5MYE2EYPFRMF4DM42G2TG3BMCDGVDL2SKYYVJDE7UN7MKM6 \
    --wasm_hash 8d45fbf03c2788e103dab335838bb71b7ab78bcc91c9d6e408bf06c88130441a


PAYGO=CASGS5MVVKEPVTCBDUW6QHCEUA532KBV4M3NQFFEF3Z5TJ5YNBR6BFID


stellar contract invoke --id $PAYGO --source-account owner
# CREATE 100 EMPLOYEES WALLETS

# UPLOAD COMPANY: upload Company contract to Stellar Blockchain

# DEPLOY USDC 

# FUND COMPANY: Owner->USDC: approve(paygo, 100)

# CREATE COMPANY: Owner->Paygo: create_company(name, description, list of employees)

# GET accountid OF COMPANY

# CALL PAY FUNCTION OF COMPANY: Owner->Company: call pay function

# VALIDATE PAYMENT TO WALLETS