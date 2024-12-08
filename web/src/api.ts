let api;

if(import.meta.env.PROD) {
    api = await import("web-lib");
}
else {
    api = await import("web-fake");
}

export const { getRecords } = api;