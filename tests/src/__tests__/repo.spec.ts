import { describe, it, expect } from '@jest/globals';
import 'dotenv/config';

const BASE_URL = process.env['BASE_URL'] || 'http://localhost:4000';
const route = '/github/repo';
const PAT_READ_ONLY = process.env['PAT_READ_ONLY'] || '';

describe('API Config', () => {
  describe('success', () => {

    it('should get API /github/repo 200', async () => {

      // Send a POST request with a body
      const responseWithBody = await fetch(`${BASE_URL}${route}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          "token": PAT_READ_ONLY,
          "org_or_user": "dfberry",
          "repo_name": "azure-notes"
        }),
      });
      expect(responseWithBody.ok).toBe(true);
      const responseBody = await responseWithBody.json();

      // Check the response status
      expect(responseWithBody.status).toBe(200);

      // Check the response headers
      expect(responseWithBody.headers.get('x-source-board-version')).toBeDefined();

      // Optionally, check the response body
      // expect(responseBody).toHaveProperty('someExpectedProperty'); // Adjust based on your API response
    });
  });

  describe('failure', () => {
    it('should get API /github/repo 400 missing param repo_name', async () => {
      try {
        const response = await fetch(`${BASE_URL}${route}`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'accept': 'application/json',
          },
          body: JSON.stringify({
            "token": PAT_READ_ONLY,
            "org_or_user": "dfberry",
            "repo_name": ""
          }),
        });

        expect(response.ok).toBe(false);

        expect(response.status).toBe(400);
      } catch (err) {
        console.error(err);
      }
    });
    it('should get API /github/repo 400 missing param org_or_user', async () => {
      try {
        const response = await fetch(`${BASE_URL}${route}`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'accept': 'application/json',
          },
          body: JSON.stringify({
            "token": PAT_READ_ONLY,
            "org_or_user": "",
            "repo_name": "azure-notes"
          }),
        });

        expect(response.ok).toBe(false);

        expect(response.status).toBe(400);
      } catch (err) {
        console.error(err);
      }
    });
    it('should get API /github/repo 401 without body', async () => {
      // Send a POST request without a body to expect a 401
      try {
        const response = await fetch(`${BASE_URL}${route}`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            "token": "",
            "org_or_user": "dfberry",
            "repo_name": "azure-notes"
          }),
        })
          ;

        expect(response.ok).toBe(false);

        expect(response.status).toBe(401);
      } catch (err) {
        console.error(err);
      }
    });
    it('should get API /github/repo 422 without token param', async () => {
      try {
        const response = await fetch(`${BASE_URL}${route}`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'accept': 'application/json',
          },
          body: JSON.stringify({
            "org_or_user": "dfberry",
            "repo_name": "azure-notes"
          }),
        });

        expect(response.ok).toBe(false);

        expect(response.status).toBe(422);
      } catch (err) {
        console.error(err);
      }
    });
  });
});