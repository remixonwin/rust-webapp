const { isValidEmail, isStrongPassword } = require('../index');

describe('Authentication Validation', () => {
  describe('Email Validation', () => {
    test('should reject invalid email format', () => {
      expect(isValidEmail('invalid-email')).toBe(false);
      expect(isValidEmail('invalid@')).toBe(false);
      expect(isValidEmail('@example.com')).toBe(false);
    });

    test('should accept valid email format', () => {
      expect(isValidEmail('test@example.com')).toBe(true);
      expect(isValidEmail('user.name+tag@example.co.uk')).toBe(true);
    });
  });

  describe('Password Validation', () => {
    test('should reject short passwords', () => {
      expect(isStrongPassword('short')).toBe(false);
      expect(isStrongPassword('123456')).toBe(false);
    });

    test('should accept strong passwords', () => {
      expect(isStrongPassword('StrongPass123!')).toBe(true);
      expect(isStrongPassword('Secure@Password2023')).toBe(true);
    });
  });
});
