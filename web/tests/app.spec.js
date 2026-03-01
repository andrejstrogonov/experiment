const { test, expect } = require('@playwright/test');

test('page loads and canvas updates after start', async ({ page }) => {
  await page.goto('/');
  // make sure UI elements exist
  await expect(page.locator('textarea')).toBeVisible();
  await expect(page.locator('button:has-text("Start")')).toBeVisible();

  // load cube script and start
  await page.click('button:has-text("Load Cube Example")');
  await page.click('button:has-text("Start")');
  // wait for some frames
  await page.waitForTimeout(500);

  // sample pixel near center of canvas
  const pixel = await page.evaluate(() => {
    const c = document.getElementById('viewport');
    const ctx = c.getContext('2d');
    const w = c.width, h = c.height;
    return ctx.getImageData(w/2, h/2, 1, 1).data;
  });
  // background color is black (#000); ensure drawing changed pixels
  expect(pixel[0] + pixel[1] + pixel[2]).toBeGreaterThan(0);
});
