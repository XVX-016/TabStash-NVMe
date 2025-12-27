# Chrome Web Store Submission Checklist

Use this checklist when submitting TabStash NVMe to the Chrome Web Store.

## Pre-Submission

- [ ] Extension is fully functional
- [ ] Native host installer is ready and tested
- [ ] All documentation is complete
- [ ] Extension has been tested on clean systems
- [ ] Icons and store images are prepared
- [ ] Privacy policy is published and accessible

## Store Listing Information

- [ ] **Name**: TabStash NVMe
- [ ] **Short description**: Written (132 chars max)
- [ ] **Detailed description**: Written and formatted
- [ ] **Category**: Selected (Productivity)
- [ ] **Language**: Selected (English)
- [ ] **Icon**: Uploaded (128x128)
- [ ] **Small tile**: Uploaded (440x280)
- [ ] **Marquee**: Uploaded (920x680)
- [ ] **Promotional images**: Optional, uploaded if available

## Privacy & Permissions

- [ ] **Privacy practices**: Completed
  - [ ] Single purpose declared
  - [ ] Data handling explained
  - [ ] User data location specified
  - [ ] Security practices described
- [ ] **Permission justifications**: Written for each permission
  - [ ] `tabs` permission explained
  - [ ] `storage` permission explained
  - [ ] `scripting` permission explained
  - [ ] `nativeMessaging` permission explained
- [ ] **Privacy policy URL**: Provided and accessible

## Distribution

- [ ] **Visibility**: Set to "Unlisted" (initially)
- [ ] **Regions**: Selected (or "All regions")
- [ ] **Pricing**: Free
- [ ] **Payment**: Not applicable

## Additional Information

- [ ] **Support URL**: GitHub issues page
- [ ] **Homepage URL**: GitHub repository
- [ ] **Privacy policy URL**: GitHub PRIVACY.md
- [ ] **Store listing language**: English

## Package Upload

- [ ] **Extension package**: Created (`.zip` file)
- [ ] **Package tested**: Loaded as unpacked extension
- [ ] **No dev files**: Removed from package
- [ ] **Manifest valid**: No errors
- [ ] **Icons present**: All referenced icons exist
- [ ] **Version number**: Set correctly (0.1.0)

## Native Messaging Disclosure

- [ ] **Native messaging usage**: Disclosed in description
- [ ] **Native host requirement**: Clearly explained
- [ ] **Installation instructions**: Provided in description and popup
- [ ] **Security explanation**: Native host security model explained

## Review Preparation

- [ ] **Reviewer notes**: Added explaining native messaging usage
- [ ] **Test instructions**: Provided for reviewers
- [ ] **Known issues**: Documented if any
- [ ] **Future plans**: Mentioned if relevant

## Post-Submission

- [ ] **Monitor review status**: Check dashboard regularly
- [ ] **Respond to questions**: Answer reviewer questions promptly
- [ ] **Address rejections**: Fix issues and resubmit if needed
- [ ] **Get extension ID**: After approval, note the extension ID
- [ ] **Update installers**: Rebuild installers with real extension ID
- [ ] **Update GitHub**: Release installers with correct extension ID

## Common Rejection Reasons (Avoid These)

- ❌ Unclear native messaging usage
- ❌ Missing permission justifications
- ❌ Vague privacy practices
- ❌ No installation instructions for native host
- ❌ Extension doesn't work without native host (but this is expected - explain clearly)
- ❌ Overbroad permissions (we're good here)
- ❌ Missing privacy policy

## Tips for Approval

1. **Be explicit**: Clearly explain why native messaging is needed
2. **Provide context**: Link to architecture documentation
3. **Show transparency**: Emphasize open source nature
4. **Explain security**: Detail native host security model
5. **Help reviewers**: Provide clear test instructions
6. **Be responsive**: Answer questions quickly

## Expected Timeline

- **Initial review**: 1-3 weeks (longer for native messaging extensions)
- **Revisions**: 1-2 weeks per iteration
- **Approval**: Total 2-4 weeks typical

## After Approval

1. Get extension ID from store dashboard
2. Update `native-host/config/tabstash-native.json.template` with real ID
3. Rebuild installers with correct extension ID
4. Test installers with published extension
5. Release installers on GitHub
6. Update extension popup links if needed
7. Consider making extension public (or keep unlisted)

