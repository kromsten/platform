

export const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

export const formatAddress = (address: string) => address.slice(0, 6) + "..." + address.slice(-4)