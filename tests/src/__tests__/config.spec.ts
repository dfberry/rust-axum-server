import { describe, it, expect } from '@jest/globals';
import 'dotenv/config';

const BACKEND_URL = process.env['BACKEND_URL'] || 'http://localhost:4000';
const route = '/config';

describe('API Config', () => {
  describe('success', () => {
    it('should get API config JSON', async () => {

      expect(process.env['ADMIN_KEY']).not.toBeUndefined();
      expect(process.env['ADMIN_KEY']?.length).toBeGreaterThan(3);

      const url = `${BACKEND_URL}${route}?admin_key=${process.env['ADMIN_KEY']}`;

      const response = await fetch(url, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
          'accept': 'application/json'
        }
      });
      expect(response.status).toBe(200);
      expect(response.ok).toBe(true);

      // Check the response headers
      expect(response.headers.get('x-source-board-version')).toBeDefined();


      // Make sure the response has app_state and env_vars properties
      const data = await response.json();
      expect(data).toHaveProperty('app_state');
      expect(data).toHaveProperty('env_vars');

    });
  });  
  describe('failure', () => {
    it('should not get API config JSON without query string, 404', async () => {

        const response = await fetch(`${BACKEND_URL}${route}`, {
          method: 'GET',
          headers: {
            'Content-Type': 'application/json'
          }
        });

        expect(response.ok).toBe(false);
        expect(response.status).toBe(404);
    });
    it('should not get API config JSON without correct value, 401', async () => {

      const response = await fetch(`${BACKEND_URL}${route}?admin_key=1234`);
      expect(response.ok).toBe(false);
      expect(response.status).toBe(401);

    });
  });
});