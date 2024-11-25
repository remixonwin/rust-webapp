const { login } = require('../index');
const { resetMockElements } = require('./setup');

describe('Login Process', () => {
  beforeEach(() => {
    resetMockElements();
    global.fetchMock.resetMocks();
  });

  describe('Successful Login', () => {
    test('should login user with valid credentials', async () => {
      // Get mock elements
      const loginEmailInput = global.globalMockElements['login-email'];
      const loginPasswordInput = global.globalMockElements['login-password'];
      const loginErrorElement = global.globalMockElements['login-error'];
      const authSection = global.globalMockElements['auth-section'];
      const userInfoSection = global.globalMockElements['user-info'];
      const emailDisplay = global.globalMockElements['email-display'];

      // Set input values
      loginEmailInput.value = 'test@example.com';
      loginPasswordInput.value = 'StrongPass123!';

      // Mock successful login response
      global.fetchMock.mockResponseOnce(JSON.stringify({ 
        token: 'fake-token', 
        email: 'test@example.com' 
      }), { status: 200 });

      // Call login function
      await login(
        loginEmailInput,
        loginPasswordInput,
        loginErrorElement,
        authSection,
        userInfoSection,
        emailDisplay
      );

      // Assertions
      expect(global.fetchMock).toHaveBeenCalledWith(
        'http://localhost:8081/api/auth/login', 
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({
            email: 'test@example.com',
            password: 'StrongPass123!'
          })
        })
      );
      expect(localStorage.setItem).toHaveBeenCalledWith('token', 'fake-token');
      expect(localStorage.setItem).toHaveBeenCalledWith('email', 'test@example.com');
      expect(userInfoSection.style.display).toBe('block');
      expect(authSection.style.display).toBe('none');
      expect(emailDisplay.textContent).toBe('test@example.com');
    });
  });

  describe('Login Error Handling', () => {
    test('should handle weak password', async () => {
      // Get mock elements
      const loginEmailInput = global.globalMockElements['login-email'];
      const loginPasswordInput = global.globalMockElements['login-password'];
      const loginErrorElement = global.globalMockElements['login-error'];

      // Set input values
      loginEmailInput.value = 'test@example.com';
      loginPasswordInput.value = 'short';

      // Call login function
      await login(
        loginEmailInput,
        loginPasswordInput,
        loginErrorElement
      );

      // Check error message
      expect(loginErrorElement.textContent).toContain('Password must be at least 8 characters long');
    });

    test('should handle invalid credentials', async () => {
      // Get mock elements
      const loginEmailInput = global.globalMockElements['login-email'];
      const loginPasswordInput = global.globalMockElements['login-password'];
      const loginErrorElement = global.globalMockElements['login-error'];

      // Set input values
      loginEmailInput.value = 'test@example.com';
      loginPasswordInput.value = 'StrongPass123!';

      // Mock server error response
      global.fetchMock.mockResponseOnce(JSON.stringify({ 
        error: 'Invalid credentials' 
      }), { status: 401 });

      // Call login function
      await login(
        loginEmailInput,
        loginPasswordInput,
        loginErrorElement
      );

      // Check error message
      expect(loginErrorElement.textContent).toContain('Invalid credentials');
    });

    test('should handle network errors during login', async () => {
      // Get mock elements
      const loginEmailInput = global.globalMockElements['login-email'];
      const loginPasswordInput = global.globalMockElements['login-password'];
      const loginErrorElement = global.globalMockElements['login-error'];

      // Set input values
      loginEmailInput.value = 'test@example.com';
      loginPasswordInput.value = 'StrongPass123!';

      // Simulate network error
      global.fetchMock.mockReject(new Error('Network error'));

      // Call login function
      await login(
        loginEmailInput,
        loginPasswordInput,
        loginErrorElement
      );

      // Check error message
      expect(loginErrorElement.textContent).toContain('Network error. Please try again.');
    });
  });
});
