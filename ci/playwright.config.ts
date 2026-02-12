import { defineConfig, devices } from '@playwright/test';

/**
 * Playwright 测试配置
 * 用于 UI 截图对比测试
 */

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [
    ['html', { open: 'never' }],
    ['json', { outputFile: 'test-results/results.json' }]
  ],
  use: {
    baseURL: 'http://localhost:8080',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
  },

  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],

  webServer: {
    command: 'cd ../frontend && trunk serve --port 8080',
    url: 'http://localhost:8080',
    reuseExistingServer: !process.env.CI,
  },
});
