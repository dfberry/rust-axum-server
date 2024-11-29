import { describe, it, expect } from '@jest/globals';
import 'dotenv/config';

const BASE_URL = process.env['BASE_URL'] || 'http://localhost:4000';
const route = '/github/repos/stats';

describe('API Repo Stat', () => {
  describe('success', () => {
    it('should get API repos/stats 200', async () => {


      // Send a POST request with a body
      const responseWithBody = await fetch(`${BASE_URL}${route}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'accept': 'application/json',
        },
        body: JSON.stringify({ "repos": ["MicrosoftDocs/node-essentials"] }),
      });

      expect(responseWithBody.ok).toBe(true);

      // Check the response status
      expect(responseWithBody.status).toBe(200);

      // Check the response headers
      expect(responseWithBody.headers.get('x-source-board-version')).toBeDefined();
    });

  });
  describe('failure', () => {
    it('should get API repos/stats 400 without body', async () => {
      // Send a POST request without a body to expect a 400 Bad Request
      try {
        const response = await fetch(`${BASE_URL}${route}`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
        });

        expect(response.ok).toBe(false);

        expect(response.status).toBe(400);
      } catch (err) {
        console.error(err);
      }
    });
  });
});