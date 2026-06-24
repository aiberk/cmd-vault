/// UI module - clean, component-based architecture for easy contributions
/// 
/// This module is structured for maximum contributor friendliness:
/// - Each component is in its own file
/// - Clear separation of concerns  
/// - Easy to add new modals/components
/// - No massive files to navigate

pub mod components;
pub mod layout;
pub mod modals;
pub mod render;

// Re-export the main drawing function for backward compatibility
pub use render::draw;