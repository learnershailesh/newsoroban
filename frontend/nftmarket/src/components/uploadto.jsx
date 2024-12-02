import axios from "axios";

// Pinata API Keys
const PINATA_API_KEY = "4052354be80279858efc";
const PINATA_SECRET_KEY = "8eeef755916492563a2fc0b8df29147f08683c7193663decc9d5879c9a8a8bd3";

// Upload metadata to IPFS using Pinata
export async function uploadToIPFS(metadata) {
  const url = `https://api.pinata.cloud/pinning/pinJSONToIPFS`;

  try {
    const response = await axios.post(url, metadata, {
      headers: {
        pinata_api_key: PINATA_API_KEY,
        pinata_secret_api_key: PINATA_SECRET_KEY,
      },
    });
    return response.data.IpfsHash; // Returns the IPFS hash
  } catch (error) {
    console.error("Error uploading to IPFS:", error);
    throw error;
  }
}
