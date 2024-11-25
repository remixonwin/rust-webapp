// Check if user is already logged in
window.onload = function() {
    const token = localStorage.getItem('token');
    if (token) {
        showUserInfo(localStorage.getItem('email'));
    }
};

// Input validation
function validateInput(email, password) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    
    if (!email || !emailRegex.test(email)) {
        return "Please enter a valid email address";
    }
    if (!password || password.length < 8) {
        return "Password must be at least 8 characters long";
    }
    if (!/[A-Z]/.test(password)) {
        return "Password must contain at least one uppercase letter";
    }
    if (!/[a-z]/.test(password)) {
        return "Password must contain at least one lowercase letter";
    }
    if (!/[0-9]/.test(password)) {
        return "Password must contain at least one number";
    }
    return null;
}

// Toggle between login and register forms
function toggleAuth() {
    const loginForm = document.getElementById('login-form');
    const registerForm = document.getElementById('register-form');
    
    if (loginForm.style.display === 'none') {
        loginForm.style.display = 'flex';
        registerForm.style.display = 'none';
    } else {
        loginForm.style.display = 'none';
        registerForm.style.display = 'flex';
    }

    // Clear error messages and form fields
    document.getElementById('login-error').textContent = '';
    document.getElementById('register-error').textContent = '';
    document.getElementById('login-email').value = '';
    document.getElementById('login-password').value = '';
    document.getElementById('register-email').value = '';
    document.getElementById('register-password').value = '';
}

// Handle user registration
async function register() {
    const email = document.getElementById('register-email').value;
    const password = document.getElementById('register-password').value;
    const errorElement = document.getElementById('register-error');

    // Client-side validation
    const validationError = validateInput(email, password);
    if (validationError) {
        errorElement.textContent = validationError;
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

        if (!response.ok) {
            throw new Error(data.error || 'Registration failed');
        }

        // Show success message and switch to login form
        errorElement.style.color = '#28a745';
        errorElement.textContent = 'Registration successful! Please login.';
        
        // Clear the form
        document.getElementById('register-email').value = '';
        document.getElementById('register-password').value = '';
        
        // Switch to login form after a delay
        setTimeout(() => {
            toggleAuth();
            errorElement.textContent = '';
            errorElement.style.color = '#dc3545';
            
            // Pre-fill the login form with the registered email
            document.getElementById('login-email').value = email;
        }, 2000);
    } catch (error) {
        errorElement.textContent = error.message;
        console.error('Registration error:', error);
    }
}

// Handle user login
async function login() {
    const email = document.getElementById('login-email').value;
    const password = document.getElementById('login-password').value;
    const errorElement = document.getElementById('login-error');

    // Basic validation
    if (!email || !password) {
        errorElement.textContent = 'Please enter both email and password';
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

        if (!response.ok) {
            throw new Error(data.error || 'Login failed');
        }

        // Store token and email
        localStorage.setItem('token', data.token);
        localStorage.setItem('email', email);
        
        // Clear form and error message
        document.getElementById('login-email').value = '';
        document.getElementById('login-password').value = '';
        errorElement.textContent = '';
        
        // Show user info
        showUserInfo(email);
    } catch (error) {
        errorElement.textContent = error.message;
        console.error('Login error:', error);
    }
}

// Handle user logout
function logout() {
    localStorage.removeItem('token');
    localStorage.removeItem('email');
    
    // Show login form
    document.getElementById('auth-section').style.display = 'block';
    document.getElementById('user-info').style.display = 'none';
    
    // Clear form fields
    document.getElementById('login-email').value = '';
    document.getElementById('login-password').value = '';
    document.getElementById('login-error').textContent = '';
}

// Show user info after successful login
function showUserInfo(email) {
    document.getElementById('auth-section').style.display = 'none';
    document.getElementById('user-info').style.display = 'block';
    document.getElementById('email-display').textContent = email;
}

// Add authentication header to all API requests
function getAuthHeader() {
    const token = localStorage.getItem('token');
    return token ? { 'Authorization': `Bearer ${token}` } : {};
}
