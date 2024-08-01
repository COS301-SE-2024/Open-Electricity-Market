// @ts-check
//note url port number can change.
import {test, expect} from '@playwright/test';

test('To login page', async ({ page }) => {
  await page.goto('http://localhost:5173');

  // Click the sign in button.
  await page.getByRole('link', { name: 'Sign in' }).click();

  // Expects to be redirected to login page.
  await expect(page).toHaveURL('http://localhost:5173/login');
});
test('To signup page', async ({ page }) => {
  await page.goto('http://localhost:5173/login');

  // Click the signup button.
  await page.getByRole('button', { name: 'Create an account' }).click();

  // Expects to be redirected to signup page.
  await expect(page).toHaveURL('http://localhost:5173/signup');
});
test('Back to login page', async ({ page }) => {
  await page.goto('http://localhost:5173/signup');

  // Click the "I already have an account" button.
  await page.getByRole('button', { name: 'I already have an account' }).click();

  // Expects to be redirected back to login page.
  await expect(page).toHaveURL('http://localhost:5173/login',{timeout:3000});
});