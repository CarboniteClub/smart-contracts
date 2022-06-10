const { sign } = require("crypto");
const nearAPI = require("near-api-js");

const { keyStores } = nearAPI;
const homedir = require("os").homedir();
const CREDENTIALS_DIR = ".near-credentials";
const credentialsPath = require("path").join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

const { connect } = nearAPI;

const DEFAULT_GAS = "30000000000000";   // 30 Terra Gas = 30 * 10^12 Gas units

const DEFAULT_DEPOSIT = "050000000000000000000000"; // 0.5 near

// Below there's an example for testnet config

const config = {
    networkId: "testnet",
    keyStore,
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://explorer.testnet.near.org",
};


// const near = await connect(config);
// const signer_account = await near.account("dev-1654805564805-31927949115487")

const contract_id = "dev-1654805564805-31927949115487";

const setup_contract = (signer_account, contract_id) => {

    const contract = new nearAPI.Contract(
        signer_account, // the account object that is connecting
        contract_id,
        {
            viewMethods: ["nft_total_supply", "nft_tokens", "nft_supply_for_owner", "nft_tokens_for_owner", "nft_metadata", "nft_token"],
            changeMethods: ["new", "new_default_meta", "nft_mint", "update_carbonite_metadata", "customize_nft"],
        }
    );

    return contract;

}

// View methods

// Returns Total number of NFT's in circulation

const nft_total_supply = (contract) => {
    return contract.nft_total_supply();
}

// Returns NFT_tokens from from_index to from_index + limit

// Eg: from_index = "458"   Note: See it's a number but it is in a string
// Eg: limit = 20 Note: It is a number itself

const nft_tokens = (contract, from_index, limit) => {
    return contract.nft_tokens({ from_index, limit });
}

// Return the otal num of  NFT's that a account has

//Eg: const account_id = "aashishyadav.testnet"

const nft_supply_for_owner = (contract, account_id) => {
    return contract.nft_supply_for_owner({ account_id });
}

// Returns Paginated View of NFT's that a account has

const nft_tokens_for_owner = (contract, account_id, from_index, limit) => {
    return contract.nft_supply_for_owner({ account_id, from_index, limit });
}

// Returns contract metadata

const nft_metadata = (contract) => {
    return contract.nft_metadata();
}

// Returns nft_token for a given token id

// const token_id = "token-1"

const nft_token = (contract, token_id) => {
    return contract.nft_token({ token_id });
}

// Change Methods

// Mint a new nft, it is actually a batch mint function so you can multiple nft's at a time

// Below nft_info_list is an Vector so you can pass multiple objects and also carbonite metadata skills and overalls field are flexible , so you can pass blockchain,design,marketing whatever you want

// total_xp is again calculated in contract

/* const nft_info_list = [
    {
        "token_id": "token-1",
        "metadata": {
            "title": "Carbonite",
            "description": "Hello! Aashish Yadav",
            "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif",
            "media_hash": "AK3YRHqKhCJNmKfV6SrutnlWW/icN5J8NUPtKsNXR1M=",
            "total_xp": 50,
            "carbonite_metadata": {
                "skills": {
                    "blockchain": 90,
                    "design": 30
                },
                "overalls": {
                    "growth": 10
                }
            }
        },
        "receiver_id": "aashishyadav.testnet"
    }
] */

const nft_mint = (contract, nft_info_list) => {
    return contract.nft_mint({
        args: {
            nft_info_list
        },
        gas: DEFAULT_GAS,
        amount: DEFAULT_DEPOSIT,
    })
}

// Updates Carbonite Metadata for a given token-id

/* Eg: const carbonite_metadata = {
    "skills": {
        "blockchain": 90,
        "design": 30
    },
    "overalls": {
        "growth": 10
    }
} */

const update_carbonite_metadata = async (contract, token_id, carbonite_metadata) => {

    return contract.update_carbonite_metadata({
        args: {
            token_id,
            carbonite_metadata
        },
        gas: DEFAULT_GAS,
        amount: DEFAULT_DEPOSIT,
    })
}

// this Just changes the picture of nft of a given token_id 
// const media = "https://crbvusujkf4437eflsv2hph2nbrm47puzpgtv4fzaf2sf3rjhcsq.arweave.net/FENaSolRec38hVyro7z6aGLOffTLzTrwuQF1Iu4pOKU?ext=png"

// const media_hash = "AK3YRHqKhCJNmKfV6SrutnlWW/icN5J8NUPtKsNXR1M="

// media hash is a base 64 representation of the SHA-256 of media file, (Doubtful whether base 64 or base 58)

const customize_nft = (contract, token_id, media, media_hash) => {
    return contract.new({
        args: {
            force,
        },
        gas: DEFAULT_GAS,
        amount: DEFAULT_DEPOSIT,
    })

}

// To get state of an account

// const new_account = await near.account(prefix + "." + contract_id);
// const state = await new_account.state();
const main = async () => {
    const near = await connect(config);
    const signer_account = await near.account("dev-1654805564805-31927949115487")
    const contract = setup_contract(signer_account, contract_id);
    const token_id = "token-2";
    const carbonite_metadata = {
        "skills": {
            "blockchain": 60,
            "design": 60
        },
        "overalls": {
            "growth": 20
        }
    }

    await update_carbonite_metadata(contract, token_id, carbonite_metadata);
}

main();
