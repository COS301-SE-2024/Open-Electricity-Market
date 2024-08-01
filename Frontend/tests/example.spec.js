// @ts-check
//note url port number can change.
import {test, expect} from '@playwright/test';

test('signup', async ({ page }) => {
  await page.goto('http://localhost:5173');

  // Click the signup button.
  await page.getByRole('link', { name: 'Sign in' }).click();

  // Expects to be redirected to signup page.
  //await expect(page).toHaveURL(/\/signup/);
  await expect(page).toHaveURL('http://localhost:5173/login',{timeout:9000});
});
