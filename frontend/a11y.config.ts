import { injectAxe, checkA11y } from 'axe-playwright';

export const a11yConfig = {
  rules: {
    'color-contrast': { enabled: true },
    'image-alt': { enabled: true },
    'label': { enabled: true },
    'button-name': { enabled: true },
    'link-name': { enabled: true },
    'aria-required-attr': { enabled: true },
    'aria-valid-attr': { enabled: true },
    'form-field-multiple-labels': { enabled: true },
    'heading-order': { enabled: true },
    'list': { enabled: true },
  },
};

export async function runA11yCheck(page: any, context?: string) {
  await injectAxe(page);
  await checkA11y(page, context, {
    detailedReport: true,
    detailedReportOptions: {
      html: true,
    },
  });
}
