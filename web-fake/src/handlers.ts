import { AppRecord } from 'web-core';

export const getRecords = async () => Promise.resolve<AppRecord[]>([
    {
        id: 1,
        name: "test 1"
    },
    {
        id: 2,
        name: "test 2"
    },
    {
        id: 3,
        name: "test 3"
    }
])