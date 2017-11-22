/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import UIKit

class ToDoListCategoryViewController: UITableViewController {

    var categories: [Category] = []

    lazy var dbStore: ToodleLib = {
        return ToodleLib.sharedInstance
    }()

    lazy var newCategoryAlertController: UIAlertController = {
        let alert = UIAlertController(title: "New Category", message: "Enter category name", preferredStyle: .alert)
        let createAction = UIAlertAction(title: "Create", style: .default, handler: { (action) in
            guard let categoryName = alert.textFields?[0].text else { return }
            self.createCategory(categoryName: categoryName)
        })
        alert.addAction(createAction)
        alert.addAction(UIAlertAction(title: "Cancel", style: .cancel, handler: { (action) in
        }))
        alert.addTextField { (textField) in
            textField.autocapitalizationType = .words
            textField.delegate = self as? UITextFieldDelegate
        }
        return alert
    }()

    override func viewDidLoad() {
        super.viewDidLoad()
        self.title = "Toodle"
        categories = self.dbStore.list.allCategories()
         self.navigationItem.rightBarButtonItem = UIBarButtonItem(barButtonSystemItem: UIBarButtonSystemItem.add, target: self, action: #selector(newCategory))
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }

    override func numberOfSections(in tableView: UITableView) -> Int {
        return 1
    }

    override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return categories.count
    }

    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "ListCell") ?? UITableViewCell(style: .subtitle, reuseIdentifier: "ListCell")
        let category = self.categories[indexPath.row]
        cell.textLabel?.text = category.name
        cell.detailTextLabel?.text = "\(category.items.count) items"
        return cell
    }

    override func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        let itemsVC = ToDoListItemsTableViewController(category: self.categories[indexPath.row])
        self.navigationController?.pushViewController(itemsVC, animated: true)
    }

    @objc fileprivate func newCategory() {
        self.present(newCategoryAlertController, animated: true)
    }

    fileprivate func createCategory(categoryName: String) {
        let category = self.dbStore.list.createCategory(withName: categoryName)
        self.categories.append(category)
        self.tableView.reloadData()
    }
}

extension ToDoListCategoryViewController: UITextViewDelegate {

    func textViewDidChange(_ textView: UITextView) {
        guard let text = textView.text,
            !text.isEmpty else {
                self.newCategoryAlertController.actions[0].isEnabled = false
                return
        }
        self.newCategoryAlertController.actions[0].isEnabled = true
    }
}
