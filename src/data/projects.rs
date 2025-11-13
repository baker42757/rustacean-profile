use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub long_description: String,
    pub technologies: Vec<String>,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub image_url: Option<String>,
}

impl Project {
    pub fn all() -> Vec<Project> {
        vec![
            Project {
                id: 1,
                title: "E-Commerce Platform".to_string(),
                description: "A full-stack e-commerce solution built with Rust and Yew".to_string(),
                long_description: "A complete e-commerce platform featuring real-time inventory management, secure payment processing, and a beautiful responsive UI. Built with Rust for maximum performance and reliability.".to_string(),
                technologies: vec!["Rust".to_string(), "Yew".to_string(), "WebAssembly".to_string(), "PostgreSQL".to_string()],
                github_url: Some("https://github.com/example/ecommerce".to_string()),
                live_url: Some("https://example.com".to_string()),
                image_url: Some("https://via.placeholder.com/600x400?text=E-Commerce".to_string()),
            },
            Project {
                id: 2,
                title: "Task Management App".to_string(),
                description: "A collaborative task management application with real-time updates".to_string(),
                long_description: "A modern task management application that allows teams to collaborate in real-time. Features include drag-and-drop organization, notifications, and seamless synchronization across devices.".to_string(),
                technologies: vec!["Rust".to_string(), "Yew".to_string(), "WebSocket".to_string(), "Tailwind CSS".to_string()],
                github_url: Some("https://github.com/example/tasks".to_string()),
                live_url: Some("https://tasks.example.com".to_string()),
                image_url: Some("https://via.placeholder.com/600x400?text=Task+Manager".to_string()),
            },
            Project {
                id: 3,
                title: "Data Visualization Dashboard".to_string(),
                description: "Interactive data visualization tool for analytics".to_string(),
                long_description: "An advanced data visualization dashboard that processes and displays complex datasets. Features interactive charts, real-time data updates, and export capabilities.".to_string(),
                technologies: vec!["Rust".to_string(), "WebAssembly".to_string(), "D3.js".to_string(), "TypeScript".to_string()],
                github_url: Some("https://github.com/example/dashboard".to_string()),
                live_url: None,
                image_url: Some("https://via.placeholder.com/600x400?text=Dashboard".to_string()),
            },
            Project {
                id: 4,
                title: "Portfolio Website".to_string(),
                description: "This very website! Built with Rust, Yew, and Tailwind CSS".to_string(),
                long_description: "A modern, responsive portfolio website showcasing projects and skills. Built entirely with Rust and WebAssembly for optimal performance and demonstrating the power of Yew framework.".to_string(),
                technologies: vec!["Rust".to_string(), "Yew".to_string(), "Tailwind CSS".to_string(), "WebAssembly".to_string()],
                github_url: Some("https://github.com/example/portfolio".to_string()),
                live_url: Some("https://portfolio.example.com".to_string()),
                image_url: None,
            },
        ]
    }
}

