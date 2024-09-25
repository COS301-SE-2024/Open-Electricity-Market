// @ts-check
import { test, expect } from "@playwright/test";
//require(‘dotenv-playwright’).config();
//import dotenv from 'dotenv';

test.describe("Landing page", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://site.localhost:5173");
  });
  test("To dashboard", async ({ page }) => {
    // Click the sign in button.
    await page.getByRole("link", { name: "Sign in" }).click();

    // Expects to be redirected to login page.
    await page.waitForURL("**/login");
  });
  test("To simulation", async ({ page }) => {
    // Click the sign in button.
    await page.getByRole("link", { name: "Simulation" }).click();

    // Expects to be redirected to login page.
    await page.waitForURL("**/public/GridSimulation");
  });
});

test.describe("simulation page (not logged in)", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://site.localhost:5173/public/GridSimulation");
  });
  test("Back to Landing page", async ({ page }) => {
    //Wait for page to finish loading
    //await page.waitForLoadState('networkidle');

    // Click the "Amplify" button.
    await page.getByRole("link", { name: "Amplify" }).click();

    // Expects to be redirected back to landing page.
    await page.waitForURL("http://site.localhost:5173");
  });
  test("To Grid Simulation page", async ({ page }) => {
    //Wait for page to finish loading
    //await page.waitForLoadState('networkidle');

    // Click the "Grid" button.
    await page.getByRole("link", { name: "Simulation" }).click();

    // Expects to be redirected to simulation grid page.
    await page.waitForURL("**/public/GridSimulation");
  });
  test("To Dashboard", async ({ page }) => {
    //Wait for page to finish loading
    //await page.waitForLoadState('networkidle');

    // Click the "Dashboard" button.
    await page.getByRole("link", { name: "Dashboard" }).click();

    // Expects to be redirected to Dashboard page.
    //await page.waitForURL("**/Main/Dashboard");
    await page.waitForURL("**/login");
  });
  test("To Analytics", async ({page}) => {
    await page.getByRole("link", {name: "Analytics" }).click();
    await page.waitForURL("**/Main/Analytics");
  });
  test("Help", async ({ page }) => {

    // Click the "Help" button.
    await page.getByRole("button", { name: "Help" }).click();

    // Expects help modal to appear.
    await expect(page.getByText("The grid simulation page contains an overview of the current state of the electrical grid. On the map, you can see all the nodes that are connected to the simulated grid. Clicking on one of these nodes will give you more information on them, and will show the voltage being generated at that point on the oscilloscope, on the right. At the bottom you can see a few general statistics about the grid")).toBeVisible();
  });
  test("To Login page", async ({page}) => {
    await page.getByRole("button", {name: "Log in" }).click();
    await page.waitForURL("**/login");
  });
});

test.describe("login page", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://site.localhost:5173/login");
  });
  test("To signup page", async ({ page }) => {
    //Wait for page to finish loading
    //await page.waitForLoadState('networkidle');

    // Click the signup button.
    await page.getByRole("link", { name: "Create an account" }).click();

    // Expects to be redirected to signup page.
    await page.waitForURL("**/signup");
  });
});

test.describe("signup page", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://site.localhost:5173/signup");
  });
  test("Back to login page", async ({ page }) => {
    //Wait for page to finish loading
    //await page.waitForLoadState('networkidle');

    // Click the "I already have an account" button.
    await page.getByRole("link", { name: "I already have an account" }).click();

    // Expects to be redirected back to login page.
    await page.waitForURL("**/login");
  });
});
test.describe("signup page error testing", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://site.localhost:5173/signup");
  });
  test("Empty email", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    //Expects an error message to appear.
    await expect(page.getByText("Invalid email or password")).toBeVisible();
  });
  test("Email no @", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in an email without an @
    await page.getByPlaceholder("Email").fill("" + process.env.EMAILNOAT);

    //Click on name field so to get page to check email Input
    await page.getByPlaceholder("First name").click();

    //Expects an error message to appear saying the email address is not valid.
    await expect(
      page.getByText("Please enter a valid email address.")
    ).toBeVisible();

    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    //Expects an error message to appear.
    await expect(page.getByText("Invalid email or password")).toBeVisible();
  });
  test("Email no suffix", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in an email without a suffix after @
    await page.getByPlaceholder("Email").fill("" + process.env.EMAILNOSUF);

    //Click on name field so to get page to check email Input
    await page.getByPlaceholder("First name").click();

    //Expects an error message to appear saying the email address is not valid.
    await expect(
      page.getByText("Please enter a valid email address.")
    ).toBeVisible();

    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    //Expects an error message to appear.
    await expect(page.getByText("Invalid email or password")).toBeVisible();
  });
  test("Email no dot", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in an email with a suffix but no dot
    await page.getByPlaceholder("Email").fill("" + process.env.EMAILNODOT);

    //Click on name field so to get page to check email Input
    await page.getByPlaceholder("First name").click();

    //Expects an error message to appear saying the email address is not valid.
    await expect(
      page.getByText("Please enter a valid email address.")
    ).toBeVisible();

    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    //Expects an error message to appear.
    await expect(page.getByText("Invalid email or password")).toBeVisible();
  });
  test("Email dot before @", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in an email that has a dot before @
    await page.getByPlaceholder("Email").fill("" + process.env.EMAILDOT);

    //Click on name field so to get page to check email Input
    await page.getByPlaceholder("First name").click();

    //Expects an error message to appear saying the email address is not valid.
    await expect(
      page.getByText("Please enter a valid email address.")
    ).toBeVisible();

    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    //Expects an error message to appear.
    await expect(page.getByText("Invalid email or password")).toBeVisible();
  });
  test("Email with no TLD", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in an email that has no TLD
    await page.getByPlaceholder("Email").fill("" + process.env.EMAILNOTLD);

    //Click on name field so to get page to check email Input
    await page.getByPlaceholder("First name").click();

    //Expects an error message to appear saying the email address is not valid.
    await expect(
      page.getByText("Please enter a valid email address.")
    ).toBeVisible();

    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    //Expects an error message to appear.
    await expect(page.getByText("Invalid email or password")).toBeVisible();
  });
  test("Empty password", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in a valid email
    await page.getByPlaceholder("Email").fill("" + process.env.EMAIL);

    //Click on name field so to get page to check email Input
    await page.getByPlaceholder("First name").click();

    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    //Expects an error message to appear.
    await expect(page.getByText("Invalid email or password")).toBeVisible();
  });
  /*test("InvalidPasswords", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in a valid email
    await page.getByPlaceholder("Email").fill("" + process.env.EMAIL);
    //Type in
    await page.getByPlaceholder("Password").fill("" + process.env.NONUM);

    //Expects an error message to appear.
    await expect(page.getByText("Password requires at least 8 characters, uppercase and lowercase, a symbol and a number")).toBeVisible();
    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    //Expects an error message to appear.
    await expect(page.getByText("Invalid email or password")).toBeVisible();
  });*/
});
