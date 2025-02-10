import { describe, it, expect } from '@jest/globals';
import 'dotenv/config';
import { stat } from 'fs';

const BACKEND_URL = process.env['BACKEND_URL'] || 'http://localhost:4000';
const route = '/github/query/issue';
const PAT_READ_ONLY = process.env['PAT_READ_ONLY'] || '';

describe('API Issue Query', () => {
    describe('success', () => {
        it('should get API 200 ' + route, async () => {

            const statusCode = 200;

            // Send a POST request with a body
            const responseWithBody = await fetch(`${BACKEND_URL}${route}`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    "token": PAT_READ_ONLY,
                    "query": "dfberry is:pr",
                }),
            });

            expect(responseWithBody.ok).toBe(true);

            // Check the response status
            expect(responseWithBody.status).toBe(statusCode);

            // Check the response headers
            expect(responseWithBody.headers.get('x-source-board-version')).toBeDefined();

         });
    });

    describe('failure', () => {
        it('should get API 400 missing param query ' + route, async () => {

            const statusCode = 400;

                const response = await fetch(`${BACKEND_URL}${route}`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'accept': 'application/json',
                    },
                    body: JSON.stringify({
                        "token": PAT_READ_ONLY,
                        "query": ""
                    }),
                });

                expect(response.ok).toBe(false);
                expect(response.status).toBe(statusCode);
                
        });

        it('should get API 400 missing post body ' + route, async () => {

            const statusCode = 400;

                const response = await fetch(`${BACKEND_URL}${route}`, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' }
                })
                expect(response.ok).toBe(false);
                expect(response.status).toBe(statusCode);
        });        
    });
});