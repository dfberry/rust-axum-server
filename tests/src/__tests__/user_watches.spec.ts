import { describe, it, expect } from '@jest/globals';
import 'dotenv/config';

const BACKEND_URL = process.env['BACKEND_URL'] || 'http://localhost:4000';
const USER_ID = process.env['USER_ID'] || 'exampleUser';
const testRepo = "dfberry/rust-axum-server";

describe('API User Watch Single Flow', () => {
  it('should create a watch, get it, and then delete it', async () => {
    // Create a new watch
    const createResponse = await fetch(`${BACKEND_URL}/user/${USER_ID}/watch`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ repo_name: testRepo }),
    });
    expect(createResponse.ok).toBe(true);
    expect(createResponse.status).toBe(201);
    const createPayload = await createResponse.json();
    expect(createPayload.repo_name).toBe(testRepo);

    // Request (GET) the watch
    const getResponse = await fetch(`${BACKEND_URL}/user/${USER_ID}/watches`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json'
      },
    });
    expect(getResponse.ok).toBe(true);
    expect(getResponse.status).toBe(200);
    const payload = await getResponse.json();
    const watches = payload.watches;
    // Assert that the created repo is found in the returned watches list.
    const foundWatch = Array.isArray(watches) ? watches.find((watch: any) => watch.repo_name === testRepo) : null;
    expect(foundWatch).not.toBeNull();
    expect(foundWatch.repo_name).toBe(testRepo);

    // Delete the watch
    const deleteResponse = await fetch(`${BACKEND_URL}/user/${USER_ID}/watch/${foundWatch.id}`, {
      method: 'DELETE',
      headers: {
        'Content-Type': 'application/json'
      }
    });
    expect(deleteResponse.ok).toBe(true);
    // TBD: why is this returning 200, NO CONTENT is 400
    expect([200, 204]).toContain(deleteResponse.status);
  });
});