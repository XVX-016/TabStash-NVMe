// TabStash NVMe Popup Script
// Handles UI updates and health checks

const INSTALLER_LINKS = {
  windows: 'https://github.com/your-repo/releases/latest/download/tabstash-installer-windows.exe',
  linux: 'https://github.com/your-repo/releases/latest',
  help: 'https://github.com/your-repo/blob/main/README.md#installation'
};

// Detect platform
function getPlatform() {
  const userAgent = navigator.userAgent.toLowerCase();
  if (userAgent.includes('win')) return 'windows';
  if (userAgent.includes('linux')) return 'linux';
  return 'unknown';
}

// Update status indicator
function updateStatus(status, message) {
  const indicator = document.getElementById('statusIndicator');
  const statusText = document.getElementById('statusText');
  const installSection = document.getElementById('installSection');
  const infoSection = document.getElementById('infoSection');
  
  // Remove all status classes
  indicator.classList.remove('connected', 'disconnected', 'checking');
  
  // Add appropriate class
  indicator.classList.add(status);
  statusText.textContent = message;
  
  // Show/hide install section
  if (status === 'disconnected') {
    installSection.classList.add('show');
    infoSection.classList.add('hidden');
    
    // Update installer links based on platform
    const platform = getPlatform();
    const windowsLink = document.getElementById('installWindows');
    const linuxLink = document.getElementById('installLinux');
    const helpLink = document.getElementById('installHelp');
    
    if (platform === 'windows') {
      windowsLink.style.display = 'block';
      linuxLink.style.display = 'none';
    } else if (platform === 'linux') {
      windowsLink.style.display = 'none';
      linuxLink.style.display = 'block';
    } else {
      windowsLink.style.display = 'block';
      linuxLink.style.display = 'block';
    }
    
    windowsLink.href = INSTALLER_LINKS.windows;
    linuxLink.href = INSTALLER_LINKS.linux;
    helpLink.href = INSTALLER_LINKS.help;
  } else {
    installSection.classList.remove('show');
    if (status === 'connected') {
      infoSection.classList.remove('hidden');
      document.getElementById('infoStatus').textContent = 'Connected';
    }
  }
}

// Perform health check
async function performHealthCheck() {
  updateStatus('checking', 'Checking connection...');
  
  try {
    // Request health check from background script
    const response = await chrome.runtime.sendMessage({ action: 'healthCheck' });
    
    if (response && response.available) {
      updateStatus('connected', 'Native host connected');
    } else {
      updateStatus('disconnected', 'Native host not available');
    }
  } catch (error) {
    console.error('Health check failed:', error);
    updateStatus('disconnected', 'Connection failed');
  }
}

// Get connection state
async function getConnectionState() {
  try {
    const response = await chrome.runtime.sendMessage({ action: 'getConnectionState' });
    return response?.state || 'unknown';
  } catch (error) {
    console.error('Failed to get connection state:', error);
    return 'unknown';
  }
}

// Load and display extension ID
async function loadExtensionId() {
  try {
    const response = await chrome.runtime.sendMessage({ action: 'getExtensionId' });
    if (response && response.extensionId) {
      const extensionIdElement = document.getElementById('extensionId');
      extensionIdElement.textContent = response.extensionId;
      
      // Setup copy button
      const copyButton = document.getElementById('copyExtensionId');
      copyButton.addEventListener('click', async () => {
        try {
          await navigator.clipboard.writeText(response.extensionId);
          copyButton.textContent = 'Copied!';
          setTimeout(() => {
            copyButton.textContent = 'Copy';
          }, 2000);
        } catch (error) {
          console.error('Failed to copy extension ID:', error);
          // Fallback: select text
          const range = document.createRange();
          range.selectNode(extensionIdElement);
          window.getSelection().removeAllRanges();
          window.getSelection().addRange(range);
          copyButton.textContent = 'Select & Copy';
        }
      });
    }
  } catch (error) {
    console.error('Failed to load extension ID:', error);
  }
}

// Initialize popup
async function init() {
  // Load extension ID first
  await loadExtensionId();
  
  // Perform health check on open
  await performHealthCheck();
  
  // Refresh status every 5 seconds
  setInterval(performHealthCheck, 5000);
}

// Run initialization when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}

