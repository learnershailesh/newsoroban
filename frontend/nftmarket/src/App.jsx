import React, { useState, } from "react";
import * as soroban from "soroban-client";
import { uploadToIPFS } from "./components/uploadto.jsx"; // Import IPFS helper
import { isConnected, getPublicKey } from "@stellar/freighter-api";



const serverUrl = "https://horizon-testnet.stellar.org"; // Testnet URL
const sorobanServer = new soroban.Server(serverUrl);

const NFTMarketplace = () => {
  const [metadata, setMetadata] = useState({ name: "", description: "" });
  const [ipfsHash, setIpfsHash] = useState("");
  const [address, setAddress] = useState(undefined);



  // Function to get the public key if connected
  let addressLookup = (async () => {
    if (await isConnected()) return getPublicKey();
  });
  const connectWallet = () => {
    if (address !== undefined) return;

    addressLookup.then((user) => {
      if (user) setAddress(user);
    });
  };



  // Mint NFT
  const mintNFT = async () => {
    try {
      // Step 1: Upload metadata to IPFS
      const ipfsHash = await uploadToIPFS(metadata);
      setIpfsHash(ipfsHash);
      console.log("Metadata uploaded to IPFS:", ipfsHash);

      // Step 2: Invoke the mint function on Soroban
      const contractId = "CCRDWI2C2IQAFCKUPVV3GRMBFP74PYO5UCMAX5H2MA77MQKNNLX2E6IX"; // Replace with your Soroban contract ID
      const contract = new soroban.Contract(contractId);

      const result = await sorobanServer.invoke({
        contract,
        method: "mint",
        params: [address, `ipfs://${ipfsHash}`], // Pass owner and IPFS hash
      });

      console.log("NFT Minted:", result);
    } catch (error) {
      console.error("Error minting NFT:", error);
    }
  };

  return (
    <div className="h-screen w-full bg-green-100">
      <h1 className="font-bold text-4xl justify-center items-center flex">NFT Marketplace</h1>

      {/* Wallet Connection */}
      <div className="flex justify-end p-4">
        <button className="bg-green-600 rounded-md text-white p-4 hover:bg-green-400" onClick={addressLookup}>Connect Wallet</button>
      </div>

      {/* Metadata Form */}

      <div className="bg-violet-300 h-80">
        <h2 className="text-2xl justify-center flex">Mint NFT</h2>
        <input className="m-4 p-4 border-solid"
          type="text"
          placeholder="Name"
          value={metadata.name}
          onChange={(e) => setMetadata({ ...metadata, name: e.target.value })}
        />
        <input className=" p-4 rounded-md"
          placeholder="Description"
          value={metadata.description}
          onChange={(e) =>
            setMetadata({ ...metadata, description: e.target.value })
          }
        />
        <input className="m-2" type="file" placeholder="image"/>
        <button className="bg-pink-400 rounded-md p-4 m-2 hover:bg-pink-300" onClick={mintNFT}>Mint NFT</button>
      </div>

      {/* Display IPFS Hash */}
      {ipfsHash && (
        <p className="justify-center flex">
          Metadata uploaded:{" "}
          <a
            href={`https://gateway.pinata.cloud/ipfs/${ipfsHash}`}
            target="_blank"
            rel="noopener noreferrer"
          >
            <span className="text-blue-600">View on IPFS</span>
          </a>
        </p>
      )}
    </div>
  );
};

export default NFTMarketplace;


