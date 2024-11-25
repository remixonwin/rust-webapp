import './styles.css';

// Authentication state
let authToken = localStorage.getItem('token');
let userEmail = localStorage.getItem('email');

// DOM Elements
const loginForm = document.getElementById('login-form');
const registerForm = document.getElementById('register-form');
const userInfoSection = document.getElementById('user-info');
const authSection = document.getElementById('auth-section');

const loginEmailInput = document.getElementById('login-email');
const loginPasswordInput = document.getElementById('login-password');
const loginButton = document.getElementById('login-button');
const loginErrorElement = document.getElementById('login-error');

const registerEmailInput = document.getElementById('register-email');
const registerPasswordInput = document.getElementById('register-password');
const registerButton = document.getElementById('register-button');
const registerErrorElement = document.getElementById('register-error');

const toggleRegisterLink = document.getElementById('toggle-register');
const toggleLoginLink = document.getElementById('toggle-login');
const logoutButton = document.getElementById('logout-button');
const emailDisplay = document.getElementById('email-display');

// Toggle between login and register forms
function toggleAuthForms() {
    if (loginForm.style.display === 'none') {
        loginForm.style.display = 'block';
        registerForm.style.display = 'none';
    } else {
        loginForm.style.display = 'none';
        registerForm.style.display = 'block';
    }
    // Clear error messages and inputs
    loginErrorElement.textContent = '';
    registerErrorElement.textContent = '';
    loginEmailInput.value = '';
    loginPasswordInput.value = '';
    registerEmailInput.value = '';
    registerPasswordInput.value = '';
}

// Email validation function
function isValidEmail(email) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
}

// Password strength validation function
function isStrongPassword(password) {
    // At least 8 characters, one uppercase, one lowercase, one number, one special character
    const passwordRegex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$/;
    return passwordRegex.test(password);
}

// Validate email and password
function validateInput(email, password) {
    if (!isValidEmail(email)) {
        return "Please enter a valid email address";
    }
    if (!isStrongPassword(password)) {
        return "Password must be at least 8 characters long, contain one uppercase letter, one lowercase letter, one number, and one special character";
    }
    return null;
}

// Show error message
function showError(errorElement, message) {
    if (!errorElement) return;

    errorElement.textContent = message;
    
    // Check if style exists before modifying
    if (errorElement.style) {
        errorElement.style.display = 'block';
    }

    setTimeout(() => {
        errorElement.textContent = '';
        
        // Check if style exists before modifying
        if (errorElement.style) {
            errorElement.style.display = 'none';
        }
    }, 3000);
}

// Login function
async function login(
  loginEmailInput = document.getElementById('login-email'),
  loginPasswordInput = document.getElementById('login-password'),
  loginErrorElement = document.getElementById('login-error'),
  authSection = document.getElementById('auth-section'),
  userInfoSection = document.getElementById('user-info'),
  emailDisplay = document.getElementById('email-display')
) {
    const email = loginEmailInput.value;
    const password = loginPasswordInput.value;

    const validationError = validateInput(email, password);
    if (validationError) {
        showError(loginErrorElement, validationError);
        return;
    }

    try {
        const response = await fetch('http://localhost:8081/api/auth/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ email, password }),
        });

        const data = await response.json();

        if (response.ok) {
            // Store token and email
            localStorage.setItem('token', data.token);
            localStorage.setItem('email', email);
            
            // Update UI
            authSection.style.display = 'none';
            userInfoSection.style.display = 'block';
            emailDisplay.textContent = email;
        } else {
            showError(loginErrorElement, data.error || 'Login failed');
        }
    } catch (error) {
        showError(loginErrorElement, 'Network error. Please try again.');
        console.error('Login error:', error);
    }
}

// Register function
async function register(
  registerEmailInput = document.getElementById('register-email'),
  registerPasswordInput = document.getElementById('register-password'),
  registerErrorElement = document.getElementById('register-error'),
  loginEmailInput = document.getElementById('login-email'),
  loginErrorElement = document.getElementById('login-error')
) {
    const email = registerEmailInput.value;
    const password = registerPasswordInput.value;

    const validationError = validateInput(email, password);
    if (validationError) {
        showError(registerErrorElement, validationError);
        return;
    }

    try {
        const response = await fetch('http://localhost:8081/api/auth/register', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ email, password }),
        });

        const data = await response.json();

        if (response.ok) {
            // Switch to login form
            toggleAuthForms();
            loginEmailInput.value = email;
            showError(loginErrorElement, 'Registration successful. Please login.');
        } else {
            showError(registerErrorElement, data.error || 'Registration failed');
        }
    } catch (error) {
        showError(registerErrorElement, 'Network error. Please try again.');
        console.error('Registration error:', error);
    }
}

// Logout function
function logout(
  userInfoSection = document.getElementById('user-info'),
  authSection = document.getElementById('auth-section'),
  loginForm = document.getElementById('login-form'),
  registerForm = document.getElementById('register-form')
) {
    // Clear local storage
    localStorage.removeItem('token');
    localStorage.removeItem('email');

    // Update UI
    userInfoSection.style.display = 'none';
    authSection.style.display = 'block';
    loginForm.style.display = 'block';
    registerForm.style.display = 'none';
}

// Check initial authentication state
function checkAuthState() {
    if (authToken) {
        authSection.style.display = 'none';
        userInfoSection.style.display = 'block';
        emailDisplay.textContent = userEmail;
    }
}

// Event Listeners
document.addEventListener('DOMContentLoaded', () => {
    // Toggle form links
    toggleRegisterLink.addEventListener('click', toggleAuthForms);
    toggleLoginLink.addEventListener('click', toggleAuthForms);

    // Authentication buttons
    loginButton.addEventListener('click', login);
    registerButton.addEventListener('click', register);
    logoutButton.addEventListener('click', logout);

    // Check initial auth state
    checkAuthState();
});

// Export functions for testing
export { 
  login, 
  register, 
  logout, 
  isValidEmail, 
  isStrongPassword 
}
