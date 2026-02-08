vim.filetype.add {
  extension = {
    ['http'] = 'http',
  },
}

vim.api.nvim_create_autocmd('FileType', {
  pattern = { 'http' },
  callback = function()
    local opt = vim.opt
    opt.number = false
    opt.preserveindent = false
    opt.relativenumber = false
  end,
})

-- REST api
-- need sudo apt install luarocks
return {
  {
    'mistweaverco/kulala.nvim',
    ft = 'http',
    keys = {
      { '<leader>r', '', desc = '  Rest' },
      { '<leader>rr', '<cmd>Rest run<cr>', desc = 'Send the request' },
      { '<leader>rs', "<cmd>lua require('kulala').run()<cr>", desc = 'Send the request' },
    },
    opts = {
      ui = {
        split_direction = 'horizontal',
        win_opts = {
          height = 25,
        },
      },
    },
  },
}
