const { register } = require('../index');
const { resetMockElements } = require('./setup');

describe('Registration Process', () => {
  beforeEach(() => {
    resetMockElements();
    global.fetchMock.resetMocks();
  });

  describe('Successful Registration', () => {
    test('should register user with valid credentials', async () => {
      // Get mock elements
      const registerEmailInput = global.globalMockElements['register-email'];
      const registerPasswordInput = global.globalMockElements['register-password'];
      const registerErrorElement = global.globalMockElements['register-error'];
      const loginEmailInput = global.globalMockElements['login-email'];
      const loginErrorElement = global.globalMockElements['login-error'];

      // Set input values
      registerEmailInput.value = 'newuser@example.com';
      registerPasswordInput.value = 'StrongPass123!';

      // Mock successful registration response
      global.fetchMock.mockResponseOnce(JSON.stringify({ 
        message: 'User registered successfully' 
      }), { status: 201 });

      // Call register function
      await register(
        registerEmailInput,
        registerPasswordInput,
        registerErrorElement,
        loginEmailInput,
        loginErrorElement
      );

      // Assertions
      expect(global.fetchMock).toHaveBeenCalledWith(
        'http://localhost:8081/api/auth/register', 
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({
            email: 'newuser@example.com',
            password: 'StrongPass123!'
          })
        })
      );
      expect(loginEmailInput.value).toBe('newuser@example.com');
    });
  });

  describe('Registration Error Handling', () => {
    test('should handle invalid email format', async () => {
      // Get mock elements
      const registerEmailInput = global.globalMockElements['register-email'];
      const registerPasswordInput = global.globalMockElements['register-password'];
      const registerErrorElement = global.globalMockElements['register-error'];

      // Set input values
      registerEmailInput.value = 'invalid-email';
      registerPasswordInput.value = 'StrongPass123!';

      // Call register function
      await register(
        registerEmailInput,
        registerPasswordInput,
        registerErrorElement
      );

      // Check error message
      expect(registerErrorElement.textContent).toContain('Please enter a valid email address');
    });

    test('should handle weak password', async () => {
      // Get mock elements
      const registerEmailInput = global.globalMockElements['register-email'];
      const registerPasswordInput = global.globalMockElements['register-password'];
      const registerErrorElement = global.globalMockElements['register-error'];

      // Set input values
      registerEmailInput.value = 'newuser@example.com';
      registerPasswordInput.value = 'short';

      // Call register function
      await register(
        registerEmailInput,
        registerPasswordInput,
        registerErrorElement
      );

      // Check error message
      expect(registerErrorElement.textContent).toContain('Password must be at least 8 characters long');
    });

    test('should handle server registration errors', async () => {
      // Get mock elements
      const registerEmailInput = global.globalMockElements['register-email'];
      const registerPasswordInput = global.globalMockElements['register-password'];
      const registerErrorElement = global.globalMockElements['register-error'];

      // Set input values
      registerEmailInput.value = 'newuser@example.com';
      registerPasswordInput.value = 'StrongPass123!';

      // Mock server error response
      global.fetchMock.mockResponseOnce(JSON.stringify({ 
        error: 'User already exists' 
      }), { status: 400 });

      // Call register function
      await register(
        registerEmailInput,
        registerPasswordInput,
        registerErrorElement
      );

      // Check error message
      expect(registerErrorElement.textContent).toContain('User already exists');
    });
  });
});
