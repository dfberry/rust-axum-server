import 'dotenv/config';
describe('Intentional Failure Test', () => {
    it('should fail intentionally', () => {

        const envFailOnPurpose = process.env['FAIL_ON_PURPOSE'];
        console.log(`FAIL_ON_PURPOSE: ${envFailOnPurpose}`);

        const failOnPurpose = envFailOnPurpose?.toLowerCase() === 'true';
        console.log(`failOnPurpose: ${failOnPurpose}`);
        console.log(`envFailOnPurpose?.toLowerCase() === 'true': ${envFailOnPurpose?.toLowerCase() === 'true'}`);

        expect(true).toBe(failOnPurpose);
    });
});