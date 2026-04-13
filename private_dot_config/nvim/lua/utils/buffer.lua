local M = {}

local ignored_filetypes = {
  Avante = true,
  AvanteConfirm = true,
  AvanteInput = true,
  AvantePromptInput = true,
  AvanteSelectedCode = true,
  AvanteSelectedFiles = true,
  AvanteTodos = true,
  spectre_panel = true,
}

local function is_sidebar_buffer(bufnr)
  if not vim.api.nvim_buf_is_valid(bufnr) then
    return false
  end

  local bo = vim.bo[bufnr]
  return ignored_filetypes[bo.filetype] or bo.buftype == 'nofile'
end

local function is_regular_buffer(bufnr, current_buf)
  if not vim.api.nvim_buf_is_valid(bufnr) or bufnr == current_buf then
    return false
  end

  local bo = vim.bo[bufnr]
  return bo.buflisted and bo.buftype == '' and not ignored_filetypes[bo.filetype]
end

local function find_replacement_buffer(current_buf)
  local alternate = vim.fn.bufnr '#'
  if is_regular_buffer(alternate, current_buf) then
    return alternate
  end

  local buffers = vim.fn.getbufinfo { buflisted = 1 }
  table.sort(buffers, function(a, b)
    return (a.lastused or 0) > (b.lastused or 0)
  end)

  for _, buffer in ipairs(buffers) do
    if is_regular_buffer(buffer.bufnr, current_buf) then
      return buffer.bufnr
    end
  end
end

function M.close_current_buffer()
  local current_buf = vim.api.nvim_get_current_buf()
  local current_win = vim.api.nvim_get_current_win()

  if is_sidebar_buffer(current_buf) then
    local ok = pcall(vim.cmd, 'close')
    if not ok and vim.api.nvim_buf_is_valid(current_buf) then
      vim.api.nvim_buf_delete(current_buf, { force = true })
    end
    return
  end

  if vim.bo[current_buf].modified then
    vim.notify('Buffer has unsaved changes', vim.log.levels.WARN)
    return
  end

  local replacement = find_replacement_buffer(current_buf)
  if replacement then
    vim.api.nvim_win_set_buf(current_win, replacement)
  else
    vim.cmd 'enew'
  end

  vim.api.nvim_buf_delete(current_buf, {})
end

return M
