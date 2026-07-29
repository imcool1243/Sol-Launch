#!/usr/bin/env python3
import base64
import hashlib
import json
from pathlib import Path
from hashlib import sha256

import requests
from solders.hash import Hash
from solders.instruction import AccountMeta, Instruction
from solders.keypair import Keypair
from solders.pubkey import Pubkey
from solders.system_program import ID as SYS_PROGRAM_ID
from solders.transaction import Transaction

PROGRAM_ID = Pubkey.from_string(
    "2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj"
)

RPC_URL = "http://127.0.0.1:8899"
WALLET_PATH = "/tmp/sol-launch-local.json"

MINT = Pubkey.from_string(
    "Fe9C3uAL4SC1QsMTeuvmakpqXBDyZu5pbMVBLKX7ihq2"
)


def discriminator(name: str) -> bytes:
    return sha256(f"global:{name}".encode()).digest()[:8]


def main():
    if not Path(WALLET_PATH).exists():
        raise SystemExit("Wallet file missing")

    secret_key = json.loads(Path(WALLET_PATH).read_text())
    wallet = Keypair.from_bytes(bytes(secret_key))

    authority = wallet.pubkey()

    launch_seed = hashlib.sha256(
        f"{authority}:launch-v1".encode()
    ).digest()

    launch = Keypair.from_seed(launch_seed)

    vault = Keypair()

    data = discriminator("initialize_vault")

    ix = Instruction(
        PROGRAM_ID,
        data,
        [
            AccountMeta(
                pubkey=authority,
                is_signer=True,
                is_writable=True,
            ),
            AccountMeta(
                pubkey=launch.pubkey(),
                is_signer=False,
                is_writable=True,
            ),
            AccountMeta(
                pubkey=MINT,
                is_signer=False,
                is_writable=False,
            ),
            AccountMeta(
                pubkey=vault.pubkey(),
                is_signer=True,
                is_writable=True,
            ),
            AccountMeta(
                pubkey=Pubkey.from_string(
                    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                ),
                is_signer=False,
                is_writable=False,
            ),
            AccountMeta(
                pubkey=SYS_PROGRAM_ID,
                is_signer=False,
                is_writable=False,
            ),
        ],
    )

    blockhash = requests.post(
        RPC_URL,
        json={
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getLatestBlockhash",
            "params": [],
        },
    ).json()["result"]["value"]["blockhash"]

    tx = Transaction.new_signed_with_payer(
        [ix],
        authority,
        [wallet, vault],
        Hash.from_string(blockhash),
    )

    response = requests.post(
        RPC_URL,
        json={
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sendTransaction",
            "params": [
                base64.b64encode(bytes(tx)).decode(),
                {
                    "encoding": "base64",
                    "preflightCommitment": "confirmed",
                },
            ],
        },
    )

    print(response.json())


if __name__ == "__main__":
    main()