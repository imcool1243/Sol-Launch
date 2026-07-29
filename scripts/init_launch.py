#!/usr/bin/env python3
import base64
import hashlib
import json
import struct
from hashlib import sha256
from pathlib import Path

import requests
from solders.hash import Hash
from solders.instruction import AccountMeta, Instruction
from solders.keypair import Keypair
from solders.pubkey import Pubkey
from solders.system_program import ID as SYS_PROGRAM_ID
from solders.transaction import Transaction

PROGRAM_ID = Pubkey.from_string("2LiNKVCp6wzftaaAmQewMjkNVzF8ztqpdrbPGqUXkhCj")
RPC_URL = "http://127.0.0.1:8899"
WALLET_PATH = "/tmp/sol-launch-local.json"


def u64_le(v: int) -> bytes:
    return struct.pack("<Q", v)


def i64_le(v: int) -> bytes:
    return struct.pack("<q", v)


def discriminator(name: str) -> bytes:
    return sha256(f"global:{name}".encode()).digest()[:8]


def main() -> None:
    if not Path(WALLET_PATH).exists():
        raise SystemExit(f"Wallet file not found: {WALLET_PATH}")

    secret_key = json.loads(Path(WALLET_PATH).read_text())
    wallet_keypair = Keypair.from_bytes(bytes(secret_key))
    authority_pubkey = wallet_keypair.pubkey()
    launch_seed = hashlib.sha256(f"{authority_pubkey}:launch-v1".encode()).digest()
    launch_keypair = Keypair.from_seed(launch_seed)
    launch_pubkey = launch_keypair.pubkey()

    data = discriminator("initialize_launch") + u64_le(100) + u64_le(500) + i64_le(30)
    ix = Instruction(
        PROGRAM_ID,
        data,
        [
            AccountMeta(pubkey=launch_pubkey, is_signer=True, is_writable=True),
            AccountMeta(pubkey=authority_pubkey, is_signer=True, is_writable=True),
            AccountMeta(pubkey=SYS_PROGRAM_ID, is_signer=False, is_writable=False),
        ],
    )

    payload = {"jsonrpc": "2.0", "id": 1, "method": "getLatestBlockhash", "params": []}
    blockhash_resp = requests.post(RPC_URL, json=payload).json()
    recent_blockhash = Hash.from_string(blockhash_resp["result"]["value"]["blockhash"])

    tx = Transaction.new_signed_with_payer(
        [ix],
        authority_pubkey,
        [wallet_keypair, launch_keypair],
        recent_blockhash,
    )

    tx_bytes = bytes(tx)
    send_payload = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "sendTransaction",
        "params": [
            base64.b64encode(tx_bytes).decode("ascii"),
            {"encoding": "base64", "preflightCommitment": "confirmed"},
        ],
    }
    sig_resp = requests.post(RPC_URL, json=send_payload).json()
    print(sig_resp)


if __name__ == "__main__":
    main()
