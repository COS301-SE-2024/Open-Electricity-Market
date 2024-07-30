// @ts-check
//note url port number can change.
import {test, expect} from '@playwright/test';

test('signup', async ({ page }) => {
  await page.goto('http://127.0.0.1:5173');

  // Click the signup button.
  await page.getByRole('button', { name: 'Create an account' }).click();

  // Expects to be redirected to signup page.
  //await expect(page).toHaveURL(/\/signup/);
  await expect(page).toHaveURL('http://127.0.0.1:5173/signup',{timeout:9000});
});
