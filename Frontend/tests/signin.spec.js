import { test, expect } from '@playwright/test';

test.describe("signed in to dashboard", () => {
    test.beforeEach(async ({ page, browserName }) => {
        test.skip(browserName === 'webkit');
        await page.goto("http://site.localhost:5173/login");
        await page.getByPlaceholder('Email').fill(process.env.EMAILFIRE);
        await page.getByPlaceholder('Password').fill(process.env.PASSWORD);
        await page.getByRole('button', { name: 'Login' }).click();
        //Wait for page to finish loading
        await page.waitForLoadState('networkidle');
        await page.waitForURL("http://site.localhost:5173/Main/Dashboard");
        await page.waitForLoadState('networkidle');
    });
    test("Back to landing page", async ({page}) => {
        await page.getByRole("link", {name: "Amplify"}).click();
        await page.waitForURL("http://site.localhost:5173");
    });
    test("Help", async ({page}) => {
        await page.getByText('Help').first().click();
        await page.getByRole('heading', { name: 'Dashboard' }).click();
        await page.getByText('This is the central hub for').click();
    });
    test("funds", async ({page}) => {
        await page.getByText('R 0.00');
        await page.getByRole('button', { name: 'Add funds' }).click();
        await page.locator('#add_modal').getByPlaceholder('Amount').click();
        await page.locator('#add_modal').getByPlaceholder('Amount').fill('1000');
        await page.locator('#add_modal').getByRole('button', { name: 'Continue' }).click();
        await page.getByRole('heading', { name: 'You have successfully added funds!' }).click();
        await page.getByText('You have successfully added R 1000.00').click();
        await page.getByText('R 1000.00').click();
    });

});
