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
function updateStatus(status, message, errorDetails = null) {
  const indicator = document.getElementById('statusIndicator');
  const statusText = document.getElementById('statusText');
  const installSection = document.getElementById('installSection');
  const infoSection = document.getElementById('infoSection');
  const errorSection = document.getElementById('errorSection');
  
  // Remove all status classes
  indicator.classList.remove('connected', 'disconnected', 'checking', 'id_mismatch');
  
  // Add appropriate class
  indicator.classList.add(status);
  statusText.textContent = message;
  
  // Hide all sections first
  installSection.classList.remove('show');
  infoSection.classList.add('hidden');
  errorSection.classList.remove('show');
  
  // Show appropriate section based on status
  if (status === 'id_mismatch' || (errorDetails && errorDetails.error)) {
    // Show error section
    errorSection.classList.add('show');
    const errorTitle = document.getElementById('errorTitle');
    const errorMessage = document.getElementById('errorMessage');
    const errorDetailsEl = document.getElementById('errorDetails');
    const errorActions = document.getElementById('errorActions');
    
    if (status === 'id_mismatch') {
      errorTitle.textContent = 'Extension ID Mismatch Detected';
      errorMessage.textContent = 'Your extension ID does not match the native host configuration.';
      
      let detailsHtml = `<strong>Your Extension ID:</strong><br>${errorDetails.extensionId || 'Unknown'}<br><br>`;
      if (errorDetails.expectedIds && errorDetails.expectedIds.length > 0) {
        detailsHtml += `<strong>Native Host Expects:</strong><br>${errorDetails.expectedIds.join(', ')}`;
      }
      errorDetailsEl.innerHTML = detailsHtml;
      
      // Show fix command
      const platform = getPlatform();
      const scriptName = platform === 'windows' ? 'link-extension.ps1' : 'link-extension.sh';
      const command = `scripts/${scriptName} "${errorDetails.extensionId || ''}"`;
      errorActions.innerHTML = `
        <div class="error-command">${command}</div>
        <p style="font-size: 11px; color: #666; margin-top: 4px;">Run this command to link your extension ID to the native host.</p>
      `;
    } else {
      errorTitle.textContent = 'Connection Error';
      errorMessage.textContent = errorDetails.error || 'Unknown error occurred';
      errorDetailsEl.textContent = '';
      errorActions.innerHTML = '';
    }
  } else if (status === 'disconnected') {
    installSection.classList.add('show');
    
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
  } else if (status === 'connected') {
    infoSection.classList.remove('hidden');
    document.getElementById('infoStatus').textContent = 'Connected';
  }
}

// Update installation status checklist
function updateInstallationStatus(status) {
  // Extension loaded (always true if popup opens)
  updateStatusItem('statusExtension', true, 'Extension loaded', null);
  
  // Native host installed
  const nativeHostInstalled = status && status.status !== 'disconnected' && status.status !== 'error';
  updateStatusItem('statusNativeHost', nativeHostInstalled, 'Native host installed', 
    nativeHostInstalled ? null : { text: 'Install', action: 'install' });
  
  // Extension ID linked
  const idLinked = status && status.status !== 'id_mismatch';
  updateStatusItem('statusIdLinked', idLinked, 'Extension ID linked',
    idLinked ? null : { text: 'Link ID', action: 'link' });
  
  // Health check passed
  const healthPassed = status && status.available === true;
  updateStatusItem('statusHealthCheck', healthPassed, 'Health check passed',
    healthPassed ? null : { text: 'Retry', action: 'retry' });
}

function updateStatusItem(itemId, success, label, action) {
  const item = document.getElementById(itemId);
  if (!item) return;
  
  item.classList.remove('success', 'error', 'checking');
  const icon = item.querySelector('.status-icon');
  const actionEl = item.querySelector('.status-action');
  
  if (success === null || success === undefined) {
    // Checking state
    item.classList.add('checking');
    icon.textContent = '⏳';
    actionEl.textContent = '';
  } else if (success) {
    // Success state
    item.classList.add('success');
    icon.textContent = '✅';
    actionEl.textContent = '';
  } else {
    // Error state
    item.classList.add('error');
    icon.textContent = '❌';
    if (action) {
      if (action.action === 'install') {
        actionEl.innerHTML = `<a href="#" class="status-action-button" onclick="window.open('https://github.com/your-repo/releases', '_blank')">${action.text}</a>`;
      } else if (action.action === 'link') {
        actionEl.innerHTML = `<span class="status-action-button" style="cursor: pointer;" onclick="copyLinkCommand()">${action.text}</span>`;
      } else if (action.action === 'retry') {
        actionEl.innerHTML = `<span class="status-action-button" style="cursor: pointer;" onclick="performHealthCheck()">${action.text}</span>`;
      }
    } else {
      actionEl.textContent = '';
    }
  }
}

// Copy link command to clipboard
async function copyLinkCommand() {
  try {
    const response = await chrome.runtime.sendMessage({ action: 'getExtensionId' });
    if (response && response.extensionId) {
      const platform = getPlatform();
      const scriptName = platform === 'windows' ? 'link-extension.ps1' : 'link-extension.sh';
      const command = `scripts/${scriptName} "${response.extensionId}"`;
      await navigator.clipboard.writeText(command);
      alert(`Command copied to clipboard:\n${command}`);
    }
  } catch (error) {
    console.error('Failed to copy link command:', error);
  }
}

// Perform health check
async function performHealthCheck() {
  updateStatus('checking', 'Checking connection...');
  updateInstallationStatus({ status: 'checking' }); // Show checking state
  
  try {
    // Request health check from background script
    const response = await chrome.runtime.sendMessage({ action: 'healthCheck' });
    
    if (response && response.available) {
      updateStatus('connected', 'Native host connected');
      updateInstallationStatus(response);
    } else if (response && response.status === 'version_mismatch') {
      updateStatus('error', 'Version mismatch', {
        error: response.error,
        protocolVersion: response.protocolVersion,
        expectedProtocolVersion: response.expectedProtocolVersion,
        nativeVersion: response.nativeVersion
      });
      updateInstallationStatus(response);
    } else if (response && response.status === 'id_mismatch') {
      updateStatus('id_mismatch', 'Extension ID mismatch', {
        error: response.error,
        extensionId: response.extensionId,
        expectedIds: response.expectedIds
      });
      updateInstallationStatus(response);
    } else {
      // Get detailed error from storage if available
      chrome.storage.local.get(['lastError'], (result) => {
        const errorDetails = result.lastError || { message: response?.error || 'Native host not available' };
        updateStatus('disconnected', errorDetails.message, { 
          error: errorDetails.message,
          details: errorDetails.details 
        });
        updateInstallationStatus(response);
      });
    }
  } catch (error) {
    console.error('Health check failed:', error);
    // Get detailed error from storage if available
    chrome.storage.local.get(['lastError'], (result) => {
      const errorDetails = result.lastError || { message: error.message };
      updateStatus('disconnected', errorDetails.message, { 
        error: errorDetails.message,
        details: errorDetails.details 
      });
      updateInstallationStatus({ status: 'error', error: errorDetails.message });
    });
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

// Load and display dev mode banner
async function loadDevModeBanner() {
  try {
    const response = await chrome.runtime.sendMessage({ action: 'getDevMode' });
    const banner = document.getElementById('devModeBanner');
    const helpLink = document.getElementById('devModeHelp');
    
    if (response && response.isDevMode) {
      banner.classList.add('show');
      helpLink.href = 'https://github.com/your-repo/blob/main/docs/DEV_INSTALL.md';
      helpLink.target = '_blank';
    } else {
      banner.classList.remove('show');
    }
  } catch (error) {
    console.error('Failed to load dev mode state:', error);
  }
}

// Initialize popup
async function init() {
  // Load extension ID first
  await loadExtensionId();
  
  // Load dev mode banner
  await loadDevModeBanner();
  
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

