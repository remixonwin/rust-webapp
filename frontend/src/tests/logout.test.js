const { logout } = require('../index');
const { resetMockElements } = require('./setup');

describe('Logout Process', () => {
  beforeEach(() => {
    resetMockElements();
    
    // Setup initial logged-in state
    localStorage.setItem('token', 'fake-token');
    localStorage.setItem('email', 'test@example.com');
  });

  describe('Successful Logout', () => {
    test('should clear user session and reset UI', () => {
      // Get mock elements
      const userInfoSection = global.globalMockElements['user-info'];
      const authSection = global.globalMockElements['auth-section'];
      const loginForm = global.globalMockElements['login-form'];
      const registerForm = global.globalMockElements['register-form'];

      // Call logout function
      logout(
        userInfoSection, 
        authSection, 
        loginForm, 
        registerForm
      );

      // Assertions
      expect(localStorage.removeItem).toHaveBeenCalledWith('token');
      expect(localStorage.removeItem).toHaveBeenCalledWith('email');
      expect(userInfoSection.style.display).toBe('none');
      expect(authSection.style.display).toBe('block');
      expect(loginForm.style.display).toBe('block');
      expect(registerForm.style.display).toBe('none');
    });
  });
});
