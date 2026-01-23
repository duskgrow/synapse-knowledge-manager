import { invoke } from '@tauri-apps/api/tauri';
import './style.css';

// Initialize the application
async function init() {
  console.log('Synapse Knowledge Manager - Frontend initialized');
  
  // Create basic UI structure
  const app = document.getElementById('app');
  if (app) {
    app.innerHTML = `
      <header style="padding: 1rem; border-bottom: 1px solid #e0e0e0;">
        <h1>Synapse Knowledge Manager</h1>
      </header>
      <main style="flex: 1; padding: 1rem;">
        <p>Editor will be integrated here with CodeMirror 6</p>
      </main>
    `;
  }
  
  // Test Tauri command
  try {
    const greeting = await invoke<string>('greet', { name: 'Synapse' });
    console.log(greeting);
  } catch (error) {
    console.error('Error calling Tauri command:', error);
  }
}

// Run initialization when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}
