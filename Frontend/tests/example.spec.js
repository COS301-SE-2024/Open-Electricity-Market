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
    await page.getByRole('link', { name: 'Show me the simulation' }).click();

    // Expects to be redirected to login page.
    await page.waitForURL('**/Main/GridSimulation');
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
    await page.getByRole('button', { name: 'Create an account' }).click();
    
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
    await page.getByRole('button', { name: 'I already have an account' }).click();

    // Expects to be redirected back to login page.
    await page.waitForURL('**/login');
  });
});
test.describe("simulation page",() => {
  test.beforeEach(async ({page})=>{
    await page.goto('http://localhost:5173/public/GridSimulation');
  });
  test('Back to Landing page', async ({ page }) => {
    //Wait for page to finish loading
    //await page.waitForLoadState('networkidle');

    // Click the "I already have an account" button.
    await page.getByRole('button', { name: 'Amplify' }).click();

    // Expects to be redirected back to login page.
    await page.waitForURL('**');
  });
});