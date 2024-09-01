import superagent from 'superagent';
import { describe, it, expect } from '@jest/globals';
import 'dotenv/config';

const BASE_URL = process.env['BASE_URL'] || 'http://localhost:4000';

describe('API Config', () => {
  it('should get API config JSON', async () => {

    const response = await superagent.get(`${BASE_URL}/config`);

    expect(response.status).toBe(200);

      // // Verify headers contain x-source-board-version
      expect(response.headers).toHaveProperty('x-source-board-version');
  
      // // Make sure the response has app_state and env_vars properties
      const data = await response.body;
      expect(data).toHaveProperty('app_state');
      expect(data).toHaveProperty('env_vars');

  });
});