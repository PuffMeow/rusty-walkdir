import { describe, expect, test } from 'vitest';

import { walkdir } from '../index';

describe('walkdir test', () => {
  test('should be defined', () => {
    expect(walkdir).toBeDefined();
  });
});
