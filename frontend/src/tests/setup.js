// Global test setup and mock utilities
const fetchMock = require('jest-fetch-mock');

// Create mock elements for testing
function createMockElements() {
    return {
        'register-email': { 
            value: '', 
            style: { display: 'block' },
            textContent: '' 
        },
        'register-password': { 
            value: '', 
            style: { display: 'block' },
            textContent: '' 
        },
        'register-confirm-password': { 
            value: '', 
            style: { display: 'block' },
            textContent: '' 
        },
        'register-error': { 
            textContent: '',
            style: { display: 'none' } 
        },
        'login-email': { 
            value: '', 
            style: { display: 'block' },
            textContent: '' 
        },
        'login-password': { 
            value: '', 
            style: { display: 'block' },
            textContent: '' 
        },
        'login-error': { 
            textContent: '',
            style: { display: 'none' } 
        },
        'login-form': { 
            style: { display: 'block' } 
        },
        'register-form': { 
            style: { display: 'block' } 
        },
        'user-info': { 
            style: { display: 'none' } 
        },
        'auth-section': { 
            style: { display: 'block' } 
        },
        'email-display': { 
            textContent: '' 
        }
    };
}

// Reset mock elements to their initial state
function resetMockElements() {
    global.globalMockElements = createMockElements();
}

// Setup before each test
beforeEach(() => {
    fetchMock.enableMocks();
    fetchMock.resetMocks();
    resetMockElements();
    
    // Create a mock localStorage
    global.localStorage = {
        _store: {},
        getItem: jest.fn().mockImplementation((key) => global.localStorage._store[key] || null),
        setItem: jest.fn().mockImplementation((key, value) => {
            global.localStorage._store[key] = value.toString();
        }),
        removeItem: jest.fn().mockImplementation((key) => {
            delete global.localStorage._store[key];
        }),
        clear: jest.fn().mockImplementation(() => {
            global.localStorage._store = {};
        })
    };

    // Create a mock document
    global.document = {
        getElementById: jest.fn((id) => {
            // Return a mock element if it exists in globalMockElements
            return global.globalMockElements[id] || null;
        })
    };

    // Set global fetchMock
    global.fetchMock = fetchMock;
});

module.exports = {
    createMockElements,
    resetMockElements
};
