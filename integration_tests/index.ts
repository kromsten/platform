type Contract = {
    wasm: Uint8Array;
    address: string;
    codeId: number;
    codeHash: string;
};


const contracts : Contract[] = [];      


const init = () => {

}


console.log("Starting integration tests...");