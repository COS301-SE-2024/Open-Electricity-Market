// @ts-check
import {test, expect} from '@playwright/test';

test.describe("Landing page",() => {
  test.beforeEach(async ({page})=>{
    await page.goto('http://localhost:5173');
  });
  test('To login page', async ({ page }) => {
    // Click the sign in button.
    await page.getByRole('link', { name: 'Sign in' }).click();

    // Expects to be redirected to login page.
    await page.waitForURL('**/login');
  });
  test('To simulation', async ({ page }) => {
    // Click the sign in button.
    await page.getByRole('link', { name: 'Simulation' }).click();

    // Expects to be redirected to login page.
    await page.waitForURL('**/public/GridSimulation');
  });
});

test.describe("public simulation page",() => {
  test.beforeEach(async ({page})=>{
    await page.goto('http://localhost:5173/public/GridSimulation');
  });
  test('Back to Landing page', async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState('networkidle');

    // Click the "Amplify" button.
    await page.getByRole('link', { name: 'Amplify' }).click();

    // Expects to be redirected back to landing page.
    await page.waitForURL('http://localhost:5173');
  });
  test('To Dashboard', async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState('networkidle');

    // Click the "Dashboard" button.
    await page.getByRole('link', { name: 'Dashboard' }).click();

    // Expects to be redirected to Dashboard page.
    await page.waitForURL('**/Main/Dashboard');
  });
  test('To public Grid Simulation page', async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState('networkidle');

    // Click the "Grid" button.
    await page.getByRole('link', { name: 'Grid' }).click();

    // Expects to be redirected to simulation grid page.
    await page.waitForURL('**/public/GridSimulation');
  });
  test('To Market page', async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState('networkidle');

    // Click the "Market" button.
    await page.getByRole('link', { name: 'Market' }).click();

    // Expects to be redirected to market page.
    await page.waitForURL('**/Main/BiddingMarket');
  });
});

test.describe("login page",() => {
  test.beforeEach(async ({page})=>{
    await page.goto('http://localhost:5173/login');
  });
  test('To signup page', async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState('networkidle');

    // Click the signup button.
    await page.getByRole('link', { name: 'Create an account' }).click();
    
    // Expects to be redirected to signup page.
    await page.waitForURL('**/signup');
  });
});

test.describe("signup page",() => {
  test.beforeEach(async ({page})=>{
    await page.goto('http://localhost:5173/signup');
  });
  test('Back to login page', async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState('networkidle');

    // Click the "I already have an account" button.
    await page.getByRole('link', { name: 'I already have an account' }).click();

    // Expects to be redirected back to login page.
    await page.waitForURL('**/login');
  });
  test('Sec:Signup with no Input', async ({ page }) => {
    //Wait for page to finish loading
    test.describe.configure({retries:1})
    await page.waitForLoadState('networkidle');

    // Click the "Create account" button.
    await page.getByRole('button', { name: 'Create account' }).click();

    //Tests if one can create an account without filling any of the forms.
    await page.waitForURL('**/login');
  });
});

