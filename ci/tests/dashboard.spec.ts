import { test, expect } from '@playwright/test';

/**
 * Dashboard UI 测试
 * 验证五栏布局和 Agent 追踪面板
 */

test.describe('Dashboard', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('五栏看板正确显示', async ({ page }) => {
    // 验证所有五栏都存在
    const columns = [
      'Todo',
      'In Progress', 
      'In Review',
      'Done',
      'Cancelled'
    ];
    
    for (const column of columns) {
      await expect(page.getByText(column, { exact: false })).toBeVisible();
    }
  });

  test('点击卡片展开 Agent 追踪面板', async ({ page }) => {
    // 点击第一个任务卡片
    const firstCard = page.locator('.task-card').first();
    await firstCard.click();
    
    // 验证右侧面板展开
    await expect(page.getByText('MISSION OBJECTIVE')).toBeVisible();
    await expect(page.getByText('PENLIGENT AGENT')).toBeVisible();
  });

  test('面板宽度为 50%', async ({ page }) => {
    // 点击卡片展开面板
    await page.locator('.task-card').first().click();
    
    // 验证面板宽度
    const panel = page.locator('[class*="AgentTrackingPanel"]').first();
    const box = await panel.boundingBox();
    
    // 获取视口宽度
    const viewport = page.viewportSize();
    
    // 面板宽度应该约为 50%
    if (box && viewport) {
      expect(box.width).toBeCloseTo(viewport.width * 0.5, -1);
    }
  });

  test('响应式布局 - 面板关闭时全宽', async ({ page }) => {
    // 验证看板区域全宽（没有面板时）
    const kanbanArea = page.locator('.kanban-column').first().locator('..');
    
    // 截图对比
    await expect(page).toHaveScreenshot('dashboard-collapsed.png', {
      maxDiffPixels: 100
    });
  });

  test('响应式布局 - 面板展开时压缩', async ({ page }) => {
    // 展开面板
    await page.locator('.task-card').first().click();
    await page.waitForTimeout(500); // 等待动画
    
    // 截图对比
    await expect(page).toHaveScreenshot('dashboard-expanded.png', {
      maxDiffPixels: 100
    });
  });
});
