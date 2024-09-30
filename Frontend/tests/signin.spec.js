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

});
