// Simple CRUD API - Pure JavaScript Example
// This demonstrates when JavaScript alone is sufficient

const express = require('express');
const cors = require('cors');
const bodyParser = require('body-parser');

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(cors());
app.use(bodyParser.json());

// In-memory database (for demo purposes)
let users = [
    { id: 1, name: 'John Doe', email: 'john@example.com', age: 30 },
    { id: 2, name: 'Jane Smith', email: 'jane@example.com', age: 25 },
    { id: 3, name: 'Bob Johnson', email: 'bob@example.com', age: 35 }
];

let nextId = 4;

// Routes

// GET /users - Get all users
app.get('/users', (req, res) => {
    console.log('📋 Getting all users...');
    res.json({
        success: true,
        data: users,
        count: users.length
    });
});

// GET /users/:id - Get user by ID
app.get('/users/:id', (req, res) => {
    const id = parseInt(req.params.id);
    console.log(`🔍 Getting user with ID: ${id}`);
    
    const user = users.find(u => u.id === id);
    if (!user) {
        return res.status(404).json({
            success: false,
            message: 'User not found'
        });
    }
    
    res.json({
        success: true,
        data: user
    });
});

// POST /users - Create new user
app.post('/users', (req, res) => {
    const { name, email, age } = req.body;
    console.log(`➕ Creating new user: ${name}`);
    
    // Simple validation
    if (!name || !email || !age) {
        return res.status(400).json({
            success: false,
            message: 'Name, email, and age are required'
        });
    }
    
    // Check if email already exists
    const existingUser = users.find(u => u.email === email);
    if (existingUser) {
        return res.status(400).json({
            success: false,
            message: 'Email already exists'
        });
    }
    
    const newUser = {
        id: nextId++,
        name,
        email,
        age: parseInt(age)
    };
    
    users.push(newUser);
    
    res.status(201).json({
        success: true,
        data: newUser,
        message: 'User created successfully'
    });
});

// PUT /users/:id - Update user
app.put('/users/:id', (req, res) => {
    const id = parseInt(req.params.id);
    const { name, email, age } = req.body;
    console.log(`✏️ Updating user with ID: ${id}`);
    
    const userIndex = users.findIndex(u => u.id === id);
    if (userIndex === -1) {
        return res.status(404).json({
            success: false,
            message: 'User not found'
        });
    }
    
    // Update user
    if (name) users[userIndex].name = name;
    if (email) users[userIndex].email = email;
    if (age) users[userIndex].age = parseInt(age);
    
    res.json({
        success: true,
        data: users[userIndex],
        message: 'User updated successfully'
    });
});

// DELETE /users/:id - Delete user
app.delete('/users/:id', (req, res) => {
    const id = parseInt(req.params.id);
    console.log(`🗑️ Deleting user with ID: ${id}`);
    
    const userIndex = users.findIndex(u => u.id === id);
    if (userIndex === -1) {
        return res.status(404).json({
            success: false,
            message: 'User not found'
        });
    }
    
    const deletedUser = users.splice(userIndex, 1)[0];
    
    res.json({
        success: true,
        data: deletedUser,
        message: 'User deleted successfully'
    });
});

// Health check
app.get('/health', (req, res) => {
    res.json({
        success: true,
        message: 'Simple CRUD API is running',
        timestamp: new Date().toISOString(),
        uptime: process.uptime()
    });
});

// Error handling middleware
app.use((err, req, res, next) => {
    console.error('❌ Error:', err);
    res.status(500).json({
        success: false,
        message: 'Internal server error'
    });
});

// 404 handler
app.use('*', (req, res) => {
    res.status(404).json({
        success: false,
        message: 'Route not found'
    });
});

// Start server
app.listen(PORT, () => {
    console.log('🚀 Simple CRUD API Server Started!');
    console.log(`📡 Server running on http://localhost:${PORT}`);
    console.log('📋 Available endpoints:');
    console.log('  GET    /users     - Get all users');
    console.log('  GET    /users/:id - Get user by ID');
    console.log('  POST   /users     - Create new user');
    console.log('  PUT    /users/:id - Update user');
    console.log('  DELETE /users/:id - Delete user');
    console.log('  GET    /health    - Health check');
    console.log('');
    console.log('💡 This is a pure JavaScript example - no Rust needed!');
    console.log('   Perfect for: CRUD operations, simple APIs, rapid prototyping');
});

module.exports = app;
