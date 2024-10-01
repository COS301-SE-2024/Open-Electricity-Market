import { test, expect } from '@playwright/test';

test.describe("signed in to dashboard", () => {
    test.beforeEach(async ({ page, browserName }) => {
        test.skip(browserName === 'webkit');
        await page.goto("/login");
        await page.getByPlaceholder('Email').fill(process.env.EMAILFIRE);
        await page.getByPlaceholder('Password').fill(process.env.PASSWORD);
        await page.getByRole('button', { name: 'Login' }).click();
        //Wait for page to finish loading
        //await page.waitForLoadState('networkidle');
        await page.waitForURL("/Main/Dashboard");
        //await page.waitForLoadState('networkidle');
    });
    test.describe("dashboard", () => {
        test("Back to landing page", async ({page}) => {
            await page.waitForLoadState('networkidle');
            await page.getByRole("link", {name: "Amplify"}).click();
            await page.waitForURL("/");
            await page.waitForLoadState('networkidle');
        });
        test("Help", async ({page}) => {
            await page.getByText('Help').first().click();
            await page.getByRole('heading', { name: 'Dashboard' }).click();
            await page.getByText('This is the central hub for').click();
        });
        test("Add funds", async ({page}) => {
            await page.getByRole('button', { name: 'Add funds' }).click();
            await page.locator('#add_modal').getByPlaceholder('Amount').fill('1000');
            await page.locator('#add_modal').getByRole('button', { name: 'Continue' }).click();
            await expect(page.getByText("You have successfully added funds!")).toBeVisible();
        });
        test("Withdraw funds", async ({page}) => {
            await page.getByRole('button', { name: 'Withdraw funds' }).click();
            await page.locator('#remove_modal').getByPlaceholder('Amount').fill('100');
            await page.locator('#remove_modal').getByRole('button', { name: 'Continue' }).click();
            await expect(page.getByText("Withdrawal of funds successful!")).toBeVisible();
        });
        test("Add a new Node", async ({page}) => {
            await page.getByRole('button', { name: 'Add a New Node' }).click();
            await page.getByPlaceholder('Name').fill('TestNode1');
            await page.locator('div').filter({ hasText: /^\+− Leaflet \| © OpenStreetMap contributors$/ }).nth(1).click();
            await page.getByRole('button', { name: 'Confirm' }).click();
            await page.getByRole('button', { name: 'Details' }).click();
            await page.getByRole('button', { name: 'Remove node' }).click();
            await page.getByRole('button', { name: 'Yes' }).click();
        });
        /*test("Add a Buy order", async ({page}) => {
            await page.getByRole('button', { name: 'Details' }).click();
            //await page.getByRole('button', { name: 'Transact with this node' }).toBeVisible();
            await page.getByRole('button', { name: 'Transact with this node' }).click();
            await page.waitForURL("/Main/BiddingMarket");
            await page.waitForLoadState('domcontentloaded');
            await page.locator("[name=buy_price]").fill("200");
            await page.locator("[name=amount]").fill("10");
            await page.getByRole('button', { name: 'Buy' }).click();
            await page.locator('#my_modal_1').getByRole('button', { name: 'Continue' }).click();
            await page.waitForURL("/Main/Dashboard");
            await page.waitForLoadState('domcontentloaded');
            await page.locator('#Capa_1').click();
            await page.locator('#cancelBuyOrder').getByRole('button', { name: 'Yes' }).click();
            
        });
        test("Add a Sell order", async ({page}) => {
            await page.getByRole('button', { name: 'Details' }).click();
            await page.getByRole('button', { name: 'Transact with this node' }).click();
            await page.waitForURL("/Main/BiddingMarket");
            await page.waitForLoadState('domcontentloaded');
            await page.locator("[name=buy_price]").fill("200");
            await page.locator("[name=amount]").fill("10");
            await page.getByRole('button', { name: 'Sell' }).click();
            await page.locator('#my_modal_2').getByRole('button', { name: 'Continue' }).click();
            await page.waitForURL("/Main/Dashboard");
            await page.waitForLoadState('domcontentloaded');
            await page.locator('#Capa_1').nth(1).click();
            await page.locator('#cancelSellOrder2').getByRole('button', { name: 'Yes' }).click();
        });*/
        /*test("remove Node", async ({page}) => {
            await page.getByRole('button', { name: 'Details' }).click();
            await page.getByRole('button', { name: 'Remove node' }).click();
            await page.getByRole('button', { name: 'Yes' }).click();
        });*/
    });
        
    
    
});
