// noinspection JSUrlImportUsage,JSFileReferences,NpmUsedModulesInstalled,JSUnresolvedReference,JSUnusedGlobalSymbols

import http from 'k6/http';
import {sleep} from 'k6';
import {expect} from "https://jslib.k6.io/k6-testing/0.5.0/index.js";

export const options = {
    vus: 100,
    duration: '1s',
};

export default function () {
    const params = {
        headers: {
            'Content-Type': 'application/json',
            'Authorization': 'Bearer Kerry Liau',
        },
    };
    f(params);
    sleep(1);
    f(params);
    sleep(1)
    f(params);
    sleep(1)
    f(params);
}

function f(params) {
    let res = http.get('http://localhost:8080/employee/users/1', params);
    expect.soft(res.status).toBe(200);
}
