import { describe, it, expect } from '@jest/globals';
import 'dotenv/config';

const BACKEND_URL = process.env['BACKEND_URL'] || 'http://localhost:4000';

async function verifyUserResults(item: string) {

    // Optionally, validate the shape of the first user object
    if (item.length > 0) {
        expect(item[0]).toHaveProperty('id');
        expect(item[0]).toHaveProperty('username');
        expect(item[0]).toHaveProperty('github_id');
        expect(item[0]).toHaveProperty('created_at');
    }

}
async function verifyWatchResults(item: string) {
    
    // Optionally, validate the shape of the first user object
    if (item.length > 0) {
        expect(item[0]).toHaveProperty('id');
        expect(item[0]).toHaveProperty('user_id');
        expect(item[0]).toHaveProperty('repo_name');
        expect(item[0]).toHaveProperty('created_at');
    }
}

async function verifySuccessfulResults(route: string, response: Response, name: string) {

    expect(response.ok).toBe(true);
    expect(response.status).toBe(200);

    // Check the response headers
    expect(response.headers.get('x-source-board-version')).toBeDefined();

    // Make sure the response has app_state and env_vars properties
    const data = await response.json();

    // Validate the shape of the response
    expect(data).toHaveProperty('request_params');
    expect(data.request_params).toHaveProperty('has_more');
    expect(data.request_params).toHaveProperty('page');
    expect(data.request_params).toHaveProperty('page_size');

    expect(data).toHaveProperty(name);
    expect(Array.isArray(data[name])).toBe(true);

    const item = data[name];
    if (name === 'users') {
        await verifyUserResults(item);
    } else if (name === 'watches') {
        await verifyWatchResults(item);
    }

}
async function requestSuccessfulRoute(route: string, name: string) {
    const response = await fetch(`${BACKEND_URL}${route}`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            'accept': 'application/json'
        }
    });
    await verifySuccessfulResults(route, response, name);
}

describe('API User', () => {
    describe('success /users', () => {
        it('should get API users JSON', async () => {
            await requestSuccessfulRoute('/users', 'users');
        });

        it('should get API users/watches JSON', async () => {
            await requestSuccessfulRoute('/users/watches', 'watches');
        });
    });
});