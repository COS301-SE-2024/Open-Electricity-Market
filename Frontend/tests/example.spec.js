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
  test("Back to landing page", async ({page}) => {
    await page.getByRole("link", {name: "Amplify"}).click();
    await page.waitForURL("http://site.localhost:5173")
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
  test("Back to landing page", async ({page}) => {
    await page.getByRole("link", {name: "Amplify"}).click();
    await page.waitForURL("http://site.localhost:5173")
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
  test("Email no prefix", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in an email with a suffix but no dot
    await page.getByPlaceholder("Email").fill("" + process.env.EMAILNOPRE);

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
  test("Email dot after @", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in an email that has a dot before @
    await page.getByPlaceholder("Email").fill("" + process.env.EMAILAT);

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
  test("Email @ before dot", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in an email that has a dot before @
    await page.getByPlaceholder("Email").fill("" + process.env.EMAILSUFAT);

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
  test("Email @ after dot", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in an email that has a dot before @
    await page.getByPlaceholder("Email").fill("" + process.env.EMAILSUFDOT);

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

  test("password with no integers", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in a valid email
    await page.getByPlaceholder("Email").fill("" + process.env.EMAIL);
    //Type in password with no numerical characters.
    await page.locator('input[placeholder="Password"]').fill("" + process.env.NONUM);

    //Expects an error message to appear.
    await expect(page.getByText("Password requires at least 8 characters, uppercase and lowercase, a symbol and a number")).toBeVisible();
    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    //Expects an error message to appear.
    await expect(page.getByText("Invalid email or password")).toBeVisible();
  });

  test("password with no symbols", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in a valid email
    await page.getByPlaceholder("Email").fill("" + process.env.EMAIL);
    //Type in password with no numerical characters.
    await page.locator('input[placeholder="Password"]').fill("" + process.env.NOSYM);

    //Expects an error message to appear.
    await expect(page.getByText("Password requires at least 8 characters, uppercase and lowercase, a symbol and a number")).toBeVisible();
    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    //Expects an error message to appear.
    await expect(page.getByText("Invalid email or password")).toBeVisible();
  });
  test("password with no capital letters", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in a valid email
    await page.getByPlaceholder("Email").fill("" + process.env.EMAIL);
    //Type in password with no numerical characters.
    await page.locator('input[placeholder="Password"]').fill("" + process.env.NOCAP);

    //Expects an error message to appear.
    await expect(page.getByText("Password requires at least 8 characters, uppercase and lowercase, a symbol and a number")).toBeVisible();
    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    //Expects an error message to appear.
    await expect(page.getByText("Invalid email or password")).toBeVisible();
  });
  test("Too short password", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in a valid email
    await page.getByPlaceholder("Email").fill("" + process.env.EMAIL);
    //Type in password with no numerical characters.
    await page.locator('input[placeholder="Password"]').fill("" + process.env.SHORT);

    //Expects an error message to appear.
    await expect(page.getByText("Password requires at least 8 characters, uppercase and lowercase, a symbol and a number")).toBeVisible();
    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    //Expects an error message to appear.
    await expect(page.getByText("Invalid email or password")).toBeVisible();
  });
  test("Not retyping password", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in a valid email
    await page.getByPlaceholder("Email").fill("" + process.env.EMAIL);
    //Type in password with no numerical characters.
    await page.locator('input[placeholder="Password"]').fill("" + process.env.PASSWORD);

    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    //Expects an error message to appear.
    await expect(page.getByText("Passwords must match")).toBeVisible();
  });
  test("Retyped password does not match", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in a valid email
    await page.getByPlaceholder("Email").fill("" + process.env.EMAIL);
    //Type in valid.
    await page.locator('input[placeholder="Password"]').fill("" + process.env.PASSWORD);

    //
    await page.locator('input[placeholder="Re-enter password"]').fill("" + process.env.SHORT);
    
    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    //Expects an error message to appear.
    await expect(page.getByText("Passwords must match")).toBeVisible();
  });

  test("All valid", async ({ page }) => {
    //Wait for page to finish loading
    await page.waitForLoadState("networkidle");

    //type in a valid email
    await page.getByPlaceholder("Email").fill("" + process.env.EMAIL);
    //Type in valid.

    await page.locator('input[placeholder="First name"]').fill("" + process.env.FIRSTNAME);

    await page.locator('input[placeholder="Surname"]').fill("" + process.env.SURNAME);

    //
    await page.locator('input[placeholder="Password"]').fill("" + process.env.PASSWORD);
    await page.locator('input[placeholder="Re-enter password"]').fill("" + process.env.PASSWORD);
    
    // Click the "Create account" button.
    await page.getByRole("button", { name: "Create account" }).click();

    await page.waitForURL("http://site.localhost:5173/Main/Dashboard");

    //await page.getByRole("button", {name: "Add funds"}).click();
    //await expect(page.getByText("Please enter an amount you would like to add.")).toBeVisible();
    //await page.getByRole("link", {name: "Analytics"}).click();
    //await page.waitForURL("http://site.localhost:5173/Main/Analytics");
  });

});
